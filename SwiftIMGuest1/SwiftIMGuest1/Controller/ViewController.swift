//
//  ViewController.swift
//  SwiftIMGuest1
//
//  Created by alan on 2024/7/16.
//

import UIKit
import SnapKit

class ViewController: UIViewController, UITableViewDataSource, UITableViewDelegate, MsgCall {
    private static var currentInstance: ViewController?
    
    // UI
    let tableView = UITableView()
    let toolbar = ToolBar();
    let ToolBarHeight: CGFloat = 44
    
    // Data
    let data = IMDataSource()
    
    override func viewDidLoad() {
        super.viewDidLoad()
        ViewController.currentInstance = self
        self.view.backgroundColor = UIColor.gray
        self.initIM()
        self.initUI()
        data.onDataUpdate = { [weak self] in
            DispatchQueue.main.async {
                self?.tableView.reloadData()
            }
        }
        setupNotificationObservers()
        self.getHistory()
    }
    
    func initIM() {
        if let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first {
            let pathString = documentsPath.path
            initIm(dbPath: pathString, id: self_id, host: host, port: portInt32, recvTopic: receive_topic, call: self)
        } else {
            print("获取沙盒文件地址出错")
        }
    }
    
    func getHistory() {
        let currentTimestamp = Int(Date().timeIntervalSince1970)
        let historyRecords = fetchLatestLimitMsgs(beforeTime: Int64(currentTimestamp), limit: 10)
        data.addRecords(records: historyRecords)
        DispatchQueue.main.async {
            self.scrollToBottom(animated: false)
        }
    }
    
    func initUI() {
        view.addSubview(tableView)
        tableView.separatorStyle = .none
        tableView.backgroundColor = UIColor.gray
        tableView.translatesAutoresizingMaskIntoConstraints = false
        tableView.dataSource = self
        tableView.delegate = self
        tableView.register(IMSendCell.self, forCellReuseIdentifier: String(describing: IMSendCell.self))
        tableView.register(IMRecvCell.self, forCellReuseIdentifier: String(describing: IMRecvCell.self))
        tableView.contentInset = UIEdgeInsets(top: 0, left: 0, bottom: ToolBarHeight + UIDevice.xx_safeDistanceBottom(), right: 0)
        self.view.addSubview(toolbar)
        
        tableView.snp.makeConstraints { make in
            make.left.right.top.bottom.equalTo(self.view);
        };
        toolbar.snp.makeConstraints { make in
            make.left.equalTo(self.view).offset(14)
            make.right.equalTo(self.view).offset(-14)
            make.bottom.equalTo(self.view).offset(-UIDevice.xx_safeDistanceBottom())
            make.height.equalTo(ToolBarHeight)
        }
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
                self.toolbar.transform = CGAffineTransform(translationX: 0, y: -keyboardHeight + self.view.safeAreaInsets.bottom)
            }
        }
    }
    
    @objc func keyboardWillHide(notification: Notification) {
        // 恢复输入框和按钮的位置
        UIView.animate(withDuration: 0.3) {
            self.toolbar.transform = .identity
        }
    }
    
    //    UITableViewDataSource
    func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        self.data.count
    }
    
    func tableView(_ tableView: UITableView, heightForRowAt indexPath: IndexPath) -> CGFloat {
        let msg_data = data.item(at: indexPath.row)
        return msg_data?.cellHeight() ?? 20
    }
    
    func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let msg_data = data.item(at: indexPath.row)
        let cls = msg_data?.getCellClass();
        let reuseIdentifier = String(describing: cls) // 使用类名作为 reuseIdentifier
        let cell: IMBasicCell
        if let dequeuedCell = tableView.dequeueReusableCell(withIdentifier: reuseIdentifier) {
            cell = dequeuedCell as! IMBasicCell
        } else {
            guard let cellType = cls else {
                fatalError("`\(String(describing: cls))` is not a subclass of UITableViewCell")
            }
            cell = cellType.init(style: .default, reuseIdentifier: reuseIdentifier)
        }
        if let record = msg_data {
            cell.setupRecord(record: record)
        }
        return cell
    }
    
    func scrollViewWillBeginDragging(_ scrollView: UIScrollView) {
        toolbar.resign()
    }
    
    func scrollToBottom(animated: Bool) {
        let lastSection = tableView.numberOfSections - 1
        if lastSection >= 0 {
            let lastRow = tableView.numberOfRows(inSection: lastSection) - 1
            if lastRow >= 0 {
                let indexPath = IndexPath(row: lastRow, section: lastSection)
                tableView.scrollToRow(at: indexPath, at: .bottom, animated: animated)
            }
        }
    }
    
    private func addMsg(record: ImModel) {
        data.addRecord(record: record)
    }
    private func addMsgs(records: [ImModel]) {
        data.addRecords(records: records)
        
    }
    func receiveMsg(record: ImModel) {
        self.addMsg(record: record)
        DispatchQueue.main.async {
            self.scrollToBottom(animated: true)
        }
    }
    
}


