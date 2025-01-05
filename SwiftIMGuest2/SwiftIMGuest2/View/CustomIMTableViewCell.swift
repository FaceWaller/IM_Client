//
//  CustomIMTableViewCell.swift
//  SwiftIMGuest1
//
//  Created by wallerface on 2025/1/4.
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
        self.selectionStyle = .none
        NSLayoutConstraint.activate([
            customLabel.centerYAnchor.constraint(equalTo: centerYAnchor),
            customLabel.leadingAnchor.constraint(equalTo: leadingAnchor, constant: 16)
        ])
    }
    
    func updateMsgData(msgData: ImModel, self_id:String) {
        let msg_from = msgData.fromId
        if self_id == msg_from  {
            customLabel.text = "发出消息: \(String(describing: msgData.msg))"
        } else {
            customLabel.text = "收到消息: \(String(describing: msgData.msg))"
        }
    }
}
