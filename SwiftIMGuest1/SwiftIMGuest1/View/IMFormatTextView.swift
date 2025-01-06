//
//  IMFormatTextView.swift
//  SwiftIMGuest1
//
//  Created by wallerface on 2025/1/6.
//

import UIKit
class IMFormatTextView: IMFormatBaseView {
    let textLabel: UILabel = {
        let textLabel = UILabel.init()
        textLabel.font = UIFont(name: "PingFang SC", size: 16)
        textLabel.textColor = UIColor.black
        textLabel.numberOfLines = 0
        
        return textLabel;
    }()
    override init(frame: CGRect) {
        super.init(frame: frame)
        self.addSubview(textLabel)
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    override func updateRecord(record: ImModel) {
        textLabel.text = record.msg
        textLabel.snp.makeConstraints { make in
            make.top.left.bottom.right.equalTo(self)
        }
        switch record.getDirection() {
        case .send:
            textLabel.textAlignment = .right
        case .recv:
            textLabel.textAlignment = .left
        }
    }
}
