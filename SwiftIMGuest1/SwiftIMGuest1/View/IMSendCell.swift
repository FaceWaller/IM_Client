//
//  IMSendCell.swift
//  SwiftIMGuest1
//
//  Created by wallerface on 2025/1/5.
//

import UIKit

class IMSendCell: IMBasicCell {
    override func setupViews() {
        super.setupViews()
        addSubview(formatView)
        icon.snp.makeConstraints { make in
            make.right.equalTo(self.contentView).offset(-14)
        };
    }
    
    override func setupRecord(record: ImModel) {
        icon.image = UIImage(named: "send")
        formatView.removeFromSuperview()
        formatView = record.formatViewClass().init()
        self.contentView.addSubview(formatView)
        formatView.snp.makeConstraints { make in
            make.left.equalTo(14)
            make.right.equalTo(icon.snp_leftMargin).offset(-14)
            make.top.equalTo(self.contentView).offset(10)
            make.bottom.equalTo(self.contentView).offset(-10)
        };
     
        formatView.updateRecord(record: record)
    }
}
