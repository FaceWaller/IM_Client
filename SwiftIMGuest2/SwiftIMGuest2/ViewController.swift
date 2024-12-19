//
//  ViewController.swift
//  SwiftIMGuest2
//
//  Created by alan on 2024/7/18.
//

import UIKit

class CustomIMTableViewCell: UITableViewCell {
    let customLabel: UILabel = {
        let label = UILabel()
        label.backgroundColor = UIColor.gray
        label.translatesAutoresizingMaskIntoConstraints = false
        return label
    }()
    
    override init(style: UITableViewCell.CellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
        setupViews()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupViews() {
        addSubview(customLabel)
        self.backgroundColor = UIColor.clear
        self.contentView.backgroundColor = UIColor.clear
        NSLayoutConstraint.activate([
            customLabel.centerYAnchor.constraint(equalTo: centerYAnchor),
            customLabel.leadingAnchor.constraint(equalTo: leadingAnchor, constant: 16)
        ])
    }
    
    func updateMsgData(msgData: [String: Any], self_id:String) {
        let msg_from = msgData["fromId"]
        let msg = msgData["msg"]
        if self_id == msg_from as! String {
            customLabel.text = "发出消息: \(String(describing: msg as! String))"
        } else {
            customLabel.text = "收到消息: \(String(describing: msg as! String))"
        }
    }
}


class ViewController: UIViewController, UITableViewDataSource, UITableViewDelegate {
    private static var currentInstance: ViewController?

    // IM config
    let self_id = "im_9528";
    let to_id = "im_9527";
    let receive_topic = "rumqtt_9528";
    let send_topic = "rumqtt_9527";
    let portInt32 = Int32(1883);
    
    // UI
    let tableView = UITableView()
    let inputTextField = UITextField()
    let sendButton = UIButton(type: .system)
    let sendBarHeight: CGFloat = 44
    let bottomOffset: CGFloat = 44
    // Data
    let data = NSMutableArray()
    
    let receiveCallback: @convention(c) (UnsafePointer<CChar>?) -> Void = { message in
        guard let message = message else { return }
        let messageString = String(cString: message)
        if let instance = ViewController.currentInstance {
            if let dict = instance.jsonStringToDictionary(messageString) {
                instance.addMsg(msgData: dict)
            }
        }
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        ViewController.currentInstance = self
        self.view.backgroundColor = UIColor.gray
        self.initIM()
        self.initUI()
        setupNotificationObservers()
        
    }
    
    func initIM() {
        let host = ("broker.emqx.io" as NSString).utf8String;
        
        if let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first {
            let pathString = documentsPath.path
            let dbPath = (pathString as NSString).utf8String
            init_im(dbPath, (self_id as NSString).utf8String, host, portInt32, (receive_topic as NSString).utf8String, receiveCallback);
        } else {
            print("获取沙盒文件地址出错")
        }
    }
    
    func initUI() {
        
        view.addSubview(tableView)
        tableView.separatorStyle = .none
        tableView.backgroundColor = UIColor.gray
        tableView.translatesAutoresizingMaskIntoConstraints = false
        NSLayoutConstraint.activate([
            tableView.topAnchor.constraint(equalTo: self.view.topAnchor),
            tableView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            tableView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            tableView.bottomAnchor.constraint(equalTo: self.view.bottomAnchor, constant: -(sendBarHeight + view.safeAreaInsets.bottom))
        ])
        tableView.dataSource = self
        tableView.delegate = self
        tableView.register(CustomIMTableViewCell.self, forCellReuseIdentifier: "CustomIMTableViewCell")
        self.setupInputComponents()
        
    }
    
    private func setupInputComponents() {
        // 设置输入框
        inputTextField.placeholder = "善语结善缘..."
        inputTextField.borderStyle = .roundedRect
        inputTextField.backgroundColor = UIColor.black
        inputTextField.textColor = UIColor.white
        inputTextField.translatesAutoresizingMaskIntoConstraints = false
        inputTextField.layer.cornerRadius = 4
        inputTextField.layer.masksToBounds = true
        
        // 设置发送按钮
        sendButton.setTitle("发送", for: .normal)
        sendButton.backgroundColor = UIColor.black
        sendButton.setTitleColor(UIColor.white, for: .normal)
        sendButton.translatesAutoresizingMaskIntoConstraints = false
        sendButton.addTarget(self, action: #selector(btnclick), for: .touchUpInside)
        sendButton.layer.cornerRadius = 4
        sendButton.layer.masksToBounds = true
        
        // 添加到视图
        view.addSubview(inputTextField)
        view.addSubview(sendButton)
        
        // 布局输入框和按钮
        NSLayoutConstraint.activate([
            inputTextField.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 14),
            inputTextField.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -bottomOffset),
            inputTextField.trailingAnchor.constraint(equalTo: sendButton.leadingAnchor, constant: -8),
            inputTextField.heightAnchor.constraint(equalToConstant: sendBarHeight),
            
            sendButton.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -14),
            sendButton.bottomAnchor.constraint(equalTo: view.bottomAnchor, constant: -bottomOffset),
            sendButton.topAnchor.constraint(equalTo: inputTextField.topAnchor),
            sendButton.widthAnchor.constraint(equalToConstant: 100),
            
        ])
    }
    
    private func setupNotificationObservers() {
        // 添加键盘出现和消失的通知观察者
        NotificationCenter.default.addObserver(self, selector: #selector(keyboardWillShow(notification:)), name: UIResponder.keyboardWillShowNotification, object: nil)
        NotificationCenter.default.addObserver(self, selector: #selector(keyboardWillHide(notification:)), name: UIResponder.keyboardWillHideNotification, object: nil)
    }
    
    @objc func keyboardWillShow(notification: Notification) {
        // 获取键盘高度
        if let keyboardFrame = notification.userInfo?[UIResponder.keyboardFrameEndUserInfoKey] as? CGRect {
            let keyboardHeight = keyboardFrame.height
            
            // 调整输入框和按钮的位置
            UIView.animate(withDuration: 0.3) {
                self.inputTextField.transform = CGAffineTransform(translationX: 0, y: -keyboardHeight + self.view.safeAreaInsets.bottom)
                self.sendButton.transform = CGAffineTransform(translationX: 0, y: -keyboardHeight + self.view.safeAreaInsets.bottom)
            }
        }
    }
    
    @objc func keyboardWillHide(notification: Notification) {
        // 恢复输入框和按钮的位置
        UIView.animate(withDuration: 0.3) {
            self.inputTextField.transform = .identity
            self.sendButton.transform = .identity
        }
    }
    
    //    UITableViewDataSource
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        self.data.count
    }
    
    func tableView(_ tableView: UITableView, heightForRowAt indexPath: IndexPath) -> CGFloat {
        60
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        guard let cell = tableView.dequeueReusableCell(withIdentifier: "CustomIMTableViewCell", for: indexPath) as? CustomIMTableViewCell else {
            return UITableViewCell()
        }
        let msg_data = data[indexPath.row]
        cell.updateMsgData(msgData: msg_data as! [String : Any], self_id: self_id)
        return cell
    }
    
    func scrollViewWillBeginDragging(_ scrollView: UIScrollView) {
        inputTextField.resignFirstResponder()
    }
    
    @objc private func btnclick() {
        if let msg = inputTextField.text {
            send_msg((self_id as NSString).utf8String, (to_id as NSString).utf8String, (send_topic as NSString).utf8String, (msg as NSString).utf8String)
            let msg_datga: [String: Any] = [
                "fromId": self_id,
                "toId": to_id,
                "time": NSDate().timeIntervalSince1970,
                "format": 0,
                "sig": self.generateRandomString(length:10),
                "msg": msg
            ];
            self.addMsg(msgData: msg_datga)
            inputTextField.text = nil
        }
    }
    
    private func addMsg(msgData: [String: Any]) {
        data.add(msgData)
        DispatchQueue.main.async {
            self.tableView.reloadData()
        }
    }
    
    private func generateRandomString(length: Int) -> String {
        let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        var randomString = ""
        
        for _ in 0..<length {
            let randomIndex = Int.random(in: 0..<characters.count)
            let character = characters[characters.index(characters.startIndex, offsetBy: randomIndex)]
            randomString.append(character)
        }
        return randomString
    }
    
    func jsonStringToDictionary(_ jsonString: String) -> [String: Any]? {
        if let jsonData = jsonString.data(using: .utf8) {
            do {
                // 将 JSON 数据解析为字典
                let dictionary = try JSONSerialization.jsonObject(with: jsonData, options: []) as? [String: Any]
                return dictionary
            } catch {
                print("Error parsing JSON: \(error)")
                return nil
            }
        }
        return nil
    }
}

