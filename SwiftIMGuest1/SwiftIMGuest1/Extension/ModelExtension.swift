//
//  ModelExtension.swift
//  SwiftIMGuest1
//
//  Created by wallerface on 2025/1/4.
//

import UIKit

enum MessageDirection {
    case send
    case recv
    //    case none 预留
}

extension ImModel {
    func cellHeight() -> CGFloat {
        switch self.format {
        case 0: do {
            let paragraphStyle = NSMutableParagraphStyle()
            paragraphStyle.lineSpacing = 10
            
            let attributes: [NSAttributedString.Key: Any] = [
                .font: UIFont.systemFont(ofSize: 16.0),
                .paragraphStyle: paragraphStyle
            ]
            let screenWidth = UIScreen.main.bounds.width
            let maxWidth = screenWidth - (14*3+44)
            let boundingRect = self.msg.boundingRect(
                with: CGSize(width: maxWidth, height: CGFloat.greatestFiniteMagnitude),
                options: [.usesLineFragmentOrigin, .usesFontLeading],
                attributes: attributes,
                context: nil
            )
            return max (ceil(boundingRect.height) + 20, 60)
        }
        default:
            return 100.0
        }
    }
    
    func getCellClass() -> IMBasicCell.Type {
        switch self.getDirection() {
        case .send:
            return IMSendCell.self
        case .recv:
            return IMRecvCell.self
        }
    }
    
    func getDirection() -> MessageDirection {
        if fromId == self_id {
            return .send
        } else {
            return .recv
        }
    }
    
    func formatViewClass() -> IMFormatBaseView.Type {
        switch self.format {
        case 0:
            return IMFormatTextView.self
        default:
            return IMFormatBaseView.self
        }
    }
    
}
