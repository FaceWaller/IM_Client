//
//  IMBasicCell.swift
//  SwiftIMGuest1
//
//  Created by wallerface on 2025/1/6.
//

import UIKit
class IMBasicCell: UITableViewCell {
    let icon: UIImageView = {
        let icon = UIImageView()
        icon.layer.cornerRadius = 8
        icon.layer.masksToBounds = true
        return icon
    }()
    lazy var formatView: IMFormatBaseView = {
        let view = IMFormatBaseView()
        return view
    }()
    required override init(style: UITableViewCell.CellStyle, reuseIdentifier: String?) {
        super.init(style: style, reuseIdentifier: reuseIdentifier)
        setupViews()
    }
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    func setupViews() {
        self.backgroundColor = UIColor.clear
        self.contentView.backgroundColor = UIColor.clear
        self.selectionStyle = .none
        
        self.contentView.addSubview(icon)
        icon.snp.makeConstraints { make in
            make.width.height.equalTo(44);
            make.top.equalTo(self.contentView).offset(10)
        };
        icon.backgroundColor = UIColor.green
        
    }
    func setupRecord(record: ImModel) {
        
    }
}
