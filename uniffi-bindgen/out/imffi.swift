// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

// swiftlint:disable all
import Foundation

// Depending on the consumer's build setup, the low-level FFI code
// might be in a separate module, or it might be compiled inline into
// this module. This is a bit of light hackery to work with both.
#if canImport(imffiFFI)
    import imffiFFI
#endif

private extension RustBuffer {
    // Allocate a new buffer, copying the contents of a `UInt8` array.
    init(bytes: [UInt8]) {
        let rbuf = bytes.withUnsafeBufferPointer { ptr in
            RustBuffer.from(ptr)
        }
        self.init(capacity: rbuf.capacity, len: rbuf.len, data: rbuf.data)
    }

    static func empty() -> RustBuffer {
        RustBuffer(capacity: 0, len: 0, data: nil)
    }

    static func from(_ ptr: UnsafeBufferPointer<UInt8>) -> RustBuffer {
        try! rustCall { ffi_imffi_rustbuffer_from_bytes(ForeignBytes(bufferPointer: ptr), $0) }
    }

    // Frees the buffer in place.
    // The buffer must not be used after this is called.
    func deallocate() {
        try! rustCall { ffi_imffi_rustbuffer_free(self, $0) }
    }
}

private extension ForeignBytes {
    init(bufferPointer: UnsafeBufferPointer<UInt8>) {
        self.init(len: Int32(bufferPointer.count), data: bufferPointer.baseAddress)
    }
}

// For every type used in the interface, we provide helper methods for conveniently
// lifting and lowering that type from C-compatible data, and for reading and writing
// values of that type in a buffer.

// Helper classes/extensions that don't change.
// Someday, this will be in a library of its own.

private extension Data {
    init(rustBuffer: RustBuffer) {
        self.init(
            bytesNoCopy: rustBuffer.data!,
            count: Int(rustBuffer.len),
            deallocator: .none
        )
    }
}

// Define reader functionality.  Normally this would be defined in a class or
// struct, but we use standalone functions instead in order to make external
// types work.
//
// With external types, one swift source file needs to be able to call the read
// method on another source file's FfiConverter, but then what visibility
// should Reader have?
// - If Reader is fileprivate, then this means the read() must also
//   be fileprivate, which doesn't work with external types.
// - If Reader is internal/public, we'll get compile errors since both source
//   files will try define the same type.
//
// Instead, the read() method and these helper functions input a tuple of data

private func createReader(data: Data) -> (data: Data, offset: Data.Index) {
    (data: data, offset: 0)
}

// Reads an integer at the current offset, in big-endian order, and advances
// the offset on success. Throws if reading the integer would move the
// offset past the end of the buffer.
private func readInt<T: FixedWidthInteger>(_ reader: inout (data: Data, offset: Data.Index)) throws -> T {
    let range = reader.offset ..< reader.offset + MemoryLayout<T>.size
    guard reader.data.count >= range.upperBound else {
        throw UniffiInternalError.bufferOverflow
    }
    if T.self == UInt8.self {
        let value = reader.data[reader.offset]
        reader.offset += 1
        return value as! T
    }
    var value: T = 0
    let _ = withUnsafeMutableBytes(of: &value) { reader.data.copyBytes(to: $0, from: range) }
    reader.offset = range.upperBound
    return value.bigEndian
}

// Reads an arbitrary number of bytes, to be used to read
// raw bytes, this is useful when lifting strings
private func readBytes(_ reader: inout (data: Data, offset: Data.Index), count: Int) throws -> [UInt8] {
    let range = reader.offset ..< (reader.offset + count)
    guard reader.data.count >= range.upperBound else {
        throw UniffiInternalError.bufferOverflow
    }
    var value = [UInt8](repeating: 0, count: count)
    value.withUnsafeMutableBufferPointer { buffer in
        reader.data.copyBytes(to: buffer, from: range)
    }
    reader.offset = range.upperBound
    return value
}

// Reads a float at the current offset.
private func readFloat(_ reader: inout (data: Data, offset: Data.Index)) throws -> Float {
    return try Float(bitPattern: readInt(&reader))
}

// Reads a float at the current offset.
private func readDouble(_ reader: inout (data: Data, offset: Data.Index)) throws -> Double {
    return try Double(bitPattern: readInt(&reader))
}

// Indicates if the offset has reached the end of the buffer.
private func hasRemaining(_ reader: (data: Data, offset: Data.Index)) -> Bool {
    return reader.offset < reader.data.count
}

// Define writer functionality.  Normally this would be defined in a class or
// struct, but we use standalone functions instead in order to make external
// types work.  See the above discussion on Readers for details.

private func createWriter() -> [UInt8] {
    return []
}

private func writeBytes<S>(_ writer: inout [UInt8], _ byteArr: S) where S: Sequence, S.Element == UInt8 {
    writer.append(contentsOf: byteArr)
}

// Writes an integer in big-endian order.
//
// Warning: make sure what you are trying to write
// is in the correct type!
private func writeInt<T: FixedWidthInteger>(_ writer: inout [UInt8], _ value: T) {
    var value = value.bigEndian
    withUnsafeBytes(of: &value) { writer.append(contentsOf: $0) }
}

private func writeFloat(_ writer: inout [UInt8], _ value: Float) {
    writeInt(&writer, value.bitPattern)
}

private func writeDouble(_ writer: inout [UInt8], _ value: Double) {
    writeInt(&writer, value.bitPattern)
}

// Protocol for types that transfer other types across the FFI. This is
// analogous to the Rust trait of the same name.
private protocol FfiConverter {
    associatedtype FfiType
    associatedtype SwiftType

    static func lift(_ value: FfiType) throws -> SwiftType
    static func lower(_ value: SwiftType) -> FfiType
    static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> SwiftType
    static func write(_ value: SwiftType, into buf: inout [UInt8])
}

// Types conforming to `Primitive` pass themselves directly over the FFI.
private protocol FfiConverterPrimitive: FfiConverter where FfiType == SwiftType {}

extension FfiConverterPrimitive {
    #if swift(>=5.8)
        @_documentation(visibility: private)
    #endif
    public static func lift(_ value: FfiType) throws -> SwiftType {
        return value
    }

    #if swift(>=5.8)
        @_documentation(visibility: private)
    #endif
    public static func lower(_ value: SwiftType) -> FfiType {
        return value
    }
}

// Types conforming to `FfiConverterRustBuffer` lift and lower into a `RustBuffer`.
// Used for complex types where it's hard to write a custom lift/lower.
private protocol FfiConverterRustBuffer: FfiConverter where FfiType == RustBuffer {}

extension FfiConverterRustBuffer {
    #if swift(>=5.8)
        @_documentation(visibility: private)
    #endif
    public static func lift(_ buf: RustBuffer) throws -> SwiftType {
        var reader = createReader(data: Data(rustBuffer: buf))
        let value = try read(from: &reader)
        if hasRemaining(reader) {
            throw UniffiInternalError.incompleteData
        }
        buf.deallocate()
        return value
    }

    #if swift(>=5.8)
        @_documentation(visibility: private)
    #endif
    public static func lower(_ value: SwiftType) -> RustBuffer {
        var writer = createWriter()
        write(value, into: &writer)
        return RustBuffer(bytes: writer)
    }
}

// An error type for FFI errors. These errors occur at the UniFFI level, not
// the library level.
private enum UniffiInternalError: LocalizedError {
    case bufferOverflow
    case incompleteData
    case unexpectedOptionalTag
    case unexpectedEnumCase
    case unexpectedNullPointer
    case unexpectedRustCallStatusCode
    case unexpectedRustCallError
    case unexpectedStaleHandle
    case rustPanic(_ message: String)

    public var errorDescription: String? {
        switch self {
        case .bufferOverflow: return "Reading the requested value would read past the end of the buffer"
        case .incompleteData: return "The buffer still has data after lifting its containing value"
        case .unexpectedOptionalTag: return "Unexpected optional tag; should be 0 or 1"
        case .unexpectedEnumCase: return "Raw enum value doesn't match any cases"
        case .unexpectedNullPointer: return "Raw pointer value was null"
        case .unexpectedRustCallStatusCode: return "Unexpected RustCallStatus code"
        case .unexpectedRustCallError: return "CALL_ERROR but no errorClass specified"
        case .unexpectedStaleHandle: return "The object in the handle map has been dropped already"
        case let .rustPanic(message): return message
        }
    }
}

private extension NSLock {
    func withLock<T>(f: () throws -> T) rethrows -> T {
        lock()
        defer { self.unlock() }
        return try f()
    }
}

private let CALL_SUCCESS: Int8 = 0
private let CALL_ERROR: Int8 = 1
private let CALL_UNEXPECTED_ERROR: Int8 = 2
private let CALL_CANCELLED: Int8 = 3

private extension RustCallStatus {
    init() {
        self.init(
            code: CALL_SUCCESS,
            errorBuf: RustBuffer(
                capacity: 0,
                len: 0,
                data: nil
            )
        )
    }
}

private func rustCall<T>(_ callback: (UnsafeMutablePointer<RustCallStatus>) -> T) throws -> T {
    let neverThrow: ((RustBuffer) throws -> Never)? = nil
    return try makeRustCall(callback, errorHandler: neverThrow)
}

private func rustCallWithError<T, E: Swift.Error>(
    _ errorHandler: @escaping (RustBuffer) throws -> E,
    _ callback: (UnsafeMutablePointer<RustCallStatus>) -> T
) throws -> T {
    try makeRustCall(callback, errorHandler: errorHandler)
}

private func makeRustCall<T, E: Swift.Error>(
    _ callback: (UnsafeMutablePointer<RustCallStatus>) -> T,
    errorHandler: ((RustBuffer) throws -> E)?
) throws -> T {
    uniffiEnsureInitialized()
    var callStatus = RustCallStatus()
    let returnedVal = callback(&callStatus)
    try uniffiCheckCallStatus(callStatus: callStatus, errorHandler: errorHandler)
    return returnedVal
}

private func uniffiCheckCallStatus<E: Swift.Error>(
    callStatus: RustCallStatus,
    errorHandler: ((RustBuffer) throws -> E)?
) throws {
    switch callStatus.code {
    case CALL_SUCCESS:
        return

    case CALL_ERROR:
        if let errorHandler = errorHandler {
            throw try errorHandler(callStatus.errorBuf)
        } else {
            callStatus.errorBuf.deallocate()
            throw UniffiInternalError.unexpectedRustCallError
        }

    case CALL_UNEXPECTED_ERROR:
        // When the rust code sees a panic, it tries to construct a RustBuffer
        // with the message.  But if that code panics, then it just sends back
        // an empty buffer.
        if callStatus.errorBuf.len > 0 {
            throw try UniffiInternalError.rustPanic(FfiConverterString.lift(callStatus.errorBuf))
        } else {
            callStatus.errorBuf.deallocate()
            throw UniffiInternalError.rustPanic("Rust panic")
        }

    case CALL_CANCELLED:
        fatalError("Cancellation not supported yet")

    default:
        throw UniffiInternalError.unexpectedRustCallStatusCode
    }
}

private func uniffiTraitInterfaceCall<T>(
    callStatus: UnsafeMutablePointer<RustCallStatus>,
    makeCall: () throws -> T,
    writeReturn: (T) -> Void
) {
    do {
        try writeReturn(makeCall())
    } catch {
        callStatus.pointee.code = CALL_UNEXPECTED_ERROR
        callStatus.pointee.errorBuf = FfiConverterString.lower(String(describing: error))
    }
}

private func uniffiTraitInterfaceCallWithError<T, E>(
    callStatus: UnsafeMutablePointer<RustCallStatus>,
    makeCall: () throws -> T,
    writeReturn: (T) -> Void,
    lowerError: (E) -> RustBuffer
) {
    do {
        try writeReturn(makeCall())
    } catch let error as E {
        callStatus.pointee.code = CALL_ERROR
        callStatus.pointee.errorBuf = lowerError(error)
    } catch {
        callStatus.pointee.code = CALL_UNEXPECTED_ERROR
        callStatus.pointee.errorBuf = FfiConverterString.lower(String(describing: error))
    }
}

private class UniffiHandleMap<T> {
    private var map: [UInt64: T] = [:]
    private let lock = NSLock()
    private var currentHandle: UInt64 = 1

    func insert(obj: T) -> UInt64 {
        lock.withLock {
            let handle = currentHandle
            currentHandle += 1
            map[handle] = obj
            return handle
        }
    }

    func get(handle: UInt64) throws -> T {
        try lock.withLock {
            guard let obj = map[handle] else {
                throw UniffiInternalError.unexpectedStaleHandle
            }
            return obj
        }
    }

    @discardableResult
    func remove(handle: UInt64) throws -> T {
        try lock.withLock {
            guard let obj = map.removeValue(forKey: handle) else {
                throw UniffiInternalError.unexpectedStaleHandle
            }
            return obj
        }
    }

    var count: Int {
        map.count
    }
}

// Public interface members begin here.

#if swift(>=5.8)
    @_documentation(visibility: private)
#endif
private struct FfiConverterInt32: FfiConverterPrimitive {
    typealias FfiType = Int32
    typealias SwiftType = Int32

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> Int32 {
        return try lift(readInt(&buf))
    }

    public static func write(_ value: Int32, into buf: inout [UInt8]) {
        writeInt(&buf, lower(value))
    }
}

#if swift(>=5.8)
    @_documentation(visibility: private)
#endif
private struct FfiConverterInt64: FfiConverterPrimitive {
    typealias FfiType = Int64
    typealias SwiftType = Int64

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> Int64 {
        return try lift(readInt(&buf))
    }

    public static func write(_ value: Int64, into buf: inout [UInt8]) {
        writeInt(&buf, lower(value))
    }
}

#if swift(>=5.8)
    @_documentation(visibility: private)
#endif
private struct FfiConverterString: FfiConverter {
    typealias SwiftType = String
    typealias FfiType = RustBuffer

    public static func lift(_ value: RustBuffer) throws -> String {
        defer {
            value.deallocate()
        }
        if value.data == nil {
            return String()
        }
        let bytes = UnsafeBufferPointer<UInt8>(start: value.data!, count: Int(value.len))
        return String(bytes: bytes, encoding: String.Encoding.utf8)!
    }

    public static func lower(_ value: String) -> RustBuffer {
        return value.utf8CString.withUnsafeBufferPointer { ptr in
            // The swift string gives us int8_t, we want uint8_t.
            ptr.withMemoryRebound(to: UInt8.self) { ptr in
                // The swift string gives us a trailing null byte, we don't want it.
                let buf = UnsafeBufferPointer(rebasing: ptr.prefix(upTo: ptr.count - 1))
                return RustBuffer.from(buf)
            }
        }
    }

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> String {
        let len: Int32 = try readInt(&buf)
        return try String(bytes: readBytes(&buf, count: Int(len)), encoding: String.Encoding.utf8)!
    }

    public static func write(_ value: String, into buf: inout [UInt8]) {
        let len = Int32(value.utf8.count)
        writeInt(&buf, len)
        writeBytes(&buf, value.utf8)
    }
}

public enum UniffiError {
    case ImError(String
    )
    case NotFound
}

#if swift(>=5.8)
    @_documentation(visibility: private)
#endif
public struct FfiConverterTypeUniffiError: FfiConverterRustBuffer {
    typealias SwiftType = UniffiError

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> UniffiError {
        let variant: Int32 = try readInt(&buf)
        switch variant {
        case 1: return try .ImError(
                FfiConverterString.read(from: &buf)
            )

        case 2: return .NotFound

        default: throw UniffiInternalError.unexpectedEnumCase
        }
    }

    public static func write(_ value: UniffiError, into buf: inout [UInt8]) {
        switch value {
        case let .ImError(v1):
            writeInt(&buf, Int32(1))
            FfiConverterString.write(v1, into: &buf)

        case .NotFound:
            writeInt(&buf, Int32(2))
        }
    }
}

extension UniffiError: Equatable, Hashable {}

extension UniffiError: Foundation.LocalizedError {
    public var errorDescription: String? {
        String(reflecting: self)
    }
}

public protocol MsgCall: AnyObject {
    func receiveMsg(record: DbInsertImModel)
}

// Magic number for the Rust proxy to call using the same mechanism as every other method,
// to free the callback once it's dropped by Rust.
private let IDX_CALLBACK_FREE: Int32 = 0
// Callback return codes
private let UNIFFI_CALLBACK_SUCCESS: Int32 = 0
private let UNIFFI_CALLBACK_ERROR: Int32 = 1
private let UNIFFI_CALLBACK_UNEXPECTED_ERROR: Int32 = 2

// Put the implementation in a struct so we don't pollute the top-level namespace
private enum UniffiCallbackInterfaceMsgCall {
    // Create the VTable using a series of closures.
    // Swift automatically converts these into C callback functions.
    static var vtable: UniffiVTableCallbackInterfaceMsgCall = .init(
        receiveMsg: { (
            uniffiHandle: UInt64,
            record: RustBuffer,
            _: UnsafeMutableRawPointer,
            uniffiCallStatus: UnsafeMutablePointer<RustCallStatus>
        ) in
            let makeCall = {
                () throws in
                guard let uniffiObj = try? FfiConverterCallbackInterfaceMsgCall.handleMap.get(handle: uniffiHandle) else {
                    throw UniffiInternalError.unexpectedStaleHandle
                }
                return try uniffiObj.receiveMsg(
                    record: FfiConverterTypeDBInsertIMModel_lift(record)
                )
            }

            let writeReturn = { () }
            uniffiTraitInterfaceCall(
                callStatus: uniffiCallStatus,
                makeCall: makeCall,
                writeReturn: writeReturn
            )
        },
        uniffiFree: { (uniffiHandle: UInt64) in
            let result = try? FfiConverterCallbackInterfaceMsgCall.handleMap.remove(handle: uniffiHandle)
            if result == nil {
                print("Uniffi callback interface MsgCall: handle missing in uniffiFree")
            }
        }
    )
}

private func uniffiCallbackInitMsgCall() {
    uniffi_imffi_fn_init_callback_vtable_msgcall(&UniffiCallbackInterfaceMsgCall.vtable)
}

// FfiConverter protocol for callback interfaces
#if swift(>=5.8)
    @_documentation(visibility: private)
#endif
private enum FfiConverterCallbackInterfaceMsgCall {
    fileprivate static var handleMap = UniffiHandleMap<MsgCall>()
}

#if swift(>=5.8)
    @_documentation(visibility: private)
#endif
extension FfiConverterCallbackInterfaceMsgCall: FfiConverter {
    typealias SwiftType = MsgCall
    typealias FfiType = UInt64

    #if swift(>=5.8)
        @_documentation(visibility: private)
    #endif
    public static func lift(_ handle: UInt64) throws -> SwiftType {
        try handleMap.get(handle: handle)
    }

    #if swift(>=5.8)
        @_documentation(visibility: private)
    #endif
    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> SwiftType {
        let handle: UInt64 = try readInt(&buf)
        return try lift(handle)
    }

    #if swift(>=5.8)
        @_documentation(visibility: private)
    #endif
    public static func lower(_ v: SwiftType) -> UInt64 {
        return handleMap.insert(obj: v)
    }

    #if swift(>=5.8)
        @_documentation(visibility: private)
    #endif
    public static func write(_ v: SwiftType, into buf: inout [UInt8]) {
        writeInt(&buf, lower(v))
    }
}

#if swift(>=5.8)
    @_documentation(visibility: private)
#endif
private struct FfiConverterSequenceTypeDBFetchIMModel: FfiConverterRustBuffer {
    typealias SwiftType = [DbFetchImModel]

    public static func write(_ value: [DbFetchImModel], into buf: inout [UInt8]) {
        let len = Int32(value.count)
        writeInt(&buf, len)
        for item in value {
            FfiConverterTypeDBFetchIMModel.write(item, into: &buf)
        }
    }

    public static func read(from buf: inout (data: Data, offset: Data.Index)) throws -> [DbFetchImModel] {
        let len: Int32 = try readInt(&buf)
        var seq = [DbFetchImModel]()
        seq.reserveCapacity(Int(len))
        for _ in 0 ..< len {
            try seq.append(FfiConverterTypeDBFetchIMModel.read(from: &buf))
        }
        return seq
    }
}

public func deleteMsg(imSid: String) throws { try rustCallWithError(FfiConverterTypeUniffiError.lift) {
    uniffi_imffi_fn_func_delete_msg(
        FfiConverterString.lower(imSid), $0
    )
}
}

public func fetchLatestLimitMsgs(beforeTime: Int64, limit: Int64) -> [DbFetchImModel] {
    return try! FfiConverterSequenceTypeDBFetchIMModel.lift(try! rustCall {
        uniffi_imffi_fn_func_fetch_latest_limit_msgs(
            FfiConverterInt64.lower(beforeTime),
            FfiConverterInt64.lower(limit), $0
        )
    })
}

public func fetchLatestMsgs(beforeTime: Int64) -> [DbFetchImModel] {
    return try! FfiConverterSequenceTypeDBFetchIMModel.lift(try! rustCall {
        uniffi_imffi_fn_func_fetch_latest_msgs(
            FfiConverterInt64.lower(beforeTime), $0
        )
    })
}

public func initIm(dbPath: String, id: String, host: String, port: Int32, recvTopic: String, call: MsgCall) { try! rustCall {
    uniffi_imffi_fn_func_init_im(
        FfiConverterString.lower(dbPath),
        FfiConverterString.lower(id),
        FfiConverterString.lower(host),
        FfiConverterInt32.lower(port),
        FfiConverterString.lower(recvTopic),
        FfiConverterCallbackInterfaceMsgCall.lower(call), $0
    )
}
}

public func sendMsg(fromId: String, toId: String, sendTopic: String, msg: String) { try! rustCall {
    uniffi_imffi_fn_func_send_msg(
        FfiConverterString.lower(fromId),
        FfiConverterString.lower(toId),
        FfiConverterString.lower(sendTopic),
        FfiConverterString.lower(msg), $0
    )
}
}

public func updateMsg(model: DbChangestImModel) throws { try rustCallWithError(FfiConverterTypeUniffiError.lift) {
    uniffi_imffi_fn_func_update_msg(
        FfiConverterTypeDBChangestIMModel_lower(model), $0
    )
}
}

private enum InitializationResult {
    case ok
    case contractVersionMismatch
    case apiChecksumMismatch
}

// Use a global variable to perform the versioning checks. Swift ensures that
// the code inside is only computed once.
private var initializationResult: InitializationResult = {
    // Get the bindings contract version from our ComponentInterface
    let bindings_contract_version = 26
    // Get the scaffolding contract version by calling the into the dylib
    let scaffolding_contract_version = ffi_imffi_uniffi_contract_version()
    if bindings_contract_version != scaffolding_contract_version {
        return InitializationResult.contractVersionMismatch
    }
    if uniffi_imffi_checksum_func_delete_msg() != 5431 {
        return InitializationResult.apiChecksumMismatch
    }
    if uniffi_imffi_checksum_func_fetch_latest_limit_msgs() != 860 {
        return InitializationResult.apiChecksumMismatch
    }
    if uniffi_imffi_checksum_func_fetch_latest_msgs() != 39385 {
        return InitializationResult.apiChecksumMismatch
    }
    if uniffi_imffi_checksum_func_init_im() != 52314 {
        return InitializationResult.apiChecksumMismatch
    }
    if uniffi_imffi_checksum_func_send_msg() != 26112 {
        return InitializationResult.apiChecksumMismatch
    }
    if uniffi_imffi_checksum_func_update_msg() != 47689 {
        return InitializationResult.apiChecksumMismatch
    }
    if uniffi_imffi_checksum_method_msgcall_receive_msg() != 25201 {
        return InitializationResult.apiChecksumMismatch
    }

    uniffiCallbackInitMsgCall()
    return InitializationResult.ok
}()

private func uniffiEnsureInitialized() {
    switch initializationResult {
    case .ok:
        break
    case .contractVersionMismatch:
        fatalError("UniFFI contract version mismatch: try cleaning and rebuilding your project")
    case .apiChecksumMismatch:
        fatalError("UniFFI API checksum mismatch: try cleaning and rebuilding your project")
    }
}

// swiftlint:enable all
