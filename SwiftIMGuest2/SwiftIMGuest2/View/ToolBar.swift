//
//  ToolBar.swift
//  SwiftIMGuest1
//
//  Created by wallerface on 2025/1/4.
//

import UIKit

class ToolBar: UIView {
    let inputTextField = UITextField()
    let sendButton = UIButton(type:.system)
    
    override init(frame: CGRect) {
        super.init(frame: frame)
        self.initUI()
    }
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    func initUI() {
        self.addSubview(inputTextField)
        self.addSubview(sendButton)
        
        // 设置输入框
        inputTextField.attributedPlaceholder = NSAttributedString(
            string: "善语结善缘...",
            attributes: [
                .foregroundColor: UIColor(hex: "#A1A1A1") // 使用自定义颜色（如16进制）
            ]
        )
        inputTextField.borderStyle = .roundedRect
        inputTextField.backgroundColor = UIColor(hex: "#2C2C2C")
        inputTextField.textColor = UIColor(hex: "#E0E0E0")
        inputTextField.translatesAutoresizingMaskIntoConstraints = false
        inputTextField.layer.cornerRadius = 4
        inputTextField.layer.masksToBounds = true
        
        // 设置发送按钮
        sendButton.setTitle("发送", for: .normal)
        sendButton.titleLabel?.font = UIFont(name: "HelveticaNeue-Bold", size: 16)
        sendButton.backgroundColor = UIColor(hex: "#FF5722")
        sendButton.setTitleColor(UIColor(hex: "#D1D1D1"), for: .normal)
        sendButton.translatesAutoresizingMaskIntoConstraints = false
        sendButton.addTarget(self, action: #selector(btnclick), for: .touchUpInside)
        sendButton.layer.cornerRadius = 4
        sendButton.layer.masksToBounds = true
        
        inputTextField.snp.makeConstraints { make in
            make.left.top.bottom.equalTo(self);
            make.right.equalTo(sendButton.snp_leftMargin).offset(-20);
        }
        sendButton.snp.makeConstraints { make in
            make.right.top.bottom.equalTo(self)
            make.width.equalTo(100)
        }
    }
    public func resign() {
        inputTextField.resignFirstResponder()
    }
    
    @objc private func btnclick() {
        if let msg = inputTextField.text {
            if (msg.isEmpty) {
                return;
            }
            sendMsg(fromId: self_id, toId: to_id, sendTopic: send_topic, msg: msg)
            inputTextField.text = nil
        }
    }
    
}
