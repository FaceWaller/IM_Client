//
//  IMDataSource.swift
//  SwiftIMGuest1
//
//  Created by wallerface on 2025/1/4.
//

import Foundation

class IMDataSource {
    private var dataArr:[ImModel] = []
    private var dataDict:[String: ImModel] = [:]
    var onDataUpdate: (() -> Void)?
    
    var sortClosure: ((ImModel, ImModel) -> Bool) = {$0.time < $1.time }
    
    
    func addRecord(record: ImModel) {
        dataDict[record.sid] = record
        dataArr.append(record)
        dataArr.sort(by: sortClosure)
        notifyUpdate()
        
    }
    func addRecords(records: [ImModel]) {
        for record in records {
            dataDict[record.sid] = record
        }
        
        // 批量更新数组并排序
        dataArr.append(contentsOf: records)
        dataArr.sort(by: sortClosure)
        
        // 通知外部数据更新
        notifyUpdate()
    }
    
    func remove(bySID sid: String) {
        // 从字典中删除
        guard let removedItem = dataDict.removeValue(forKey: sid) else { return }
        
        // 从数组中删除
        if let index = dataArr.firstIndex(where: { $0.sid == removedItem.sid }) {
            dataArr.remove(at: index)
        }
        notifyUpdate()
        
    }
    
    func search(bySID sid: String) -> ImModel? {
        return dataDict[sid]
    }
    
    func item(at index: Int) -> ImModel? {
        guard index >= 0 && index < dataArr.count else { return nil }
        return dataArr[index]
    }
    
    var count: Int {
        return dataArr.count
    }
    
    private func notifyUpdate() {
        onDataUpdate?()
    }
    
}
