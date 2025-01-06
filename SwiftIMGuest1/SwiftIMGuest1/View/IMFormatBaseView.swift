//
//  IMFormatBaseView.swift
//  SwiftIMGuest1
//
//  Created by wallerface on 2025/1/6.
//

import UIKit
class IMFormatBaseView : UIView {
    override init(frame: CGRect) {
        super.init(frame: frame)
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    func updateRecord(record: ImModel) {
        
    }
    
}
