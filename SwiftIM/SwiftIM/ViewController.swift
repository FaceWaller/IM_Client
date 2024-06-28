//
//  ViewController.swift
//  SwiftIM
//
//  Created by alan on 2024/6/27.
//

import UIKit



class ViewController: UIViewController {
    let self_id = ("im_9527" as NSString).utf8String;
    let to_id = ("im_9528" as NSString).utf8String;
    let topic = "rumqtt_9527";
    let portInt32 = Int32(1883);
    
    let data = NSMutableArray()
    
    // Swift callback function with C calling convention
    let receiveCallback: @convention(c) (UnsafePointer<CChar>?) -> Void = { message in
        if let message = message {
            let messageString = String(cString: message)
            print("Received message: \(messageString)")
            /// 增加数据
        }
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        self.view.backgroundColor = UIColor.gray
        
        let host = ("broker.emqx.io" as NSString).utf8String;
     
        if let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first {
            let pathString = documentsPath.path
            let dbPath = (pathString as NSString).utf8String
            init_im(dbPath, self_id, host, portInt32, (topic as NSString).utf8String, receiveCallback);
        } else {
            print("获取沙盒文件地址出错")
        }
        
        let btn = UIButton(frame: CGRect(x: 100, y: 100, width: 100, height: 100))
        self.view.addSubview(btn)
        btn.setTitle("发送", for: .normal)
        btn.setTitleColor(.white, for: .normal)
        btn.backgroundColor = UIColor.black
        btn.addTarget(self, action: #selector(btnclick), for: .touchUpInside)
    }
    
    @objc func btnclick() {
        print("点击发送")
        let msg = ("早安午安晚安")
        send_msg(self_id, to_id, (topic as NSString).utf8String, (msg as NSString).utf8String)
    }
}
