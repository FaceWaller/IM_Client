一直阅读文档并学习函数的使用，进展肯定不如实际动手来得迅速，比如实现一个 IM（即时消息）客户端；因此，我撰写了这篇实际操作文档

IM在移动互联网中是一个非常重要的能力，IM不仅提供了快速、实时的通信渠道，还可以支持多媒体文件的传输、群聊、语音通话和视频会议等功能。这使得它成为了各种应用和平台的核心功能。掌握如何创建和定制自己的IM客户端是一项非常有价值的技能，不仅可以为您的个人项目增色不少，还可以在职业发展中开辟新的机会。

#### 运行方式

1. 打包rust，cd进入ffi文件夹，执行打包命令
   模拟器：   cargo lipo --targets aarch64-apple-ios-sim
   真机：       cargo lipo --targets aarch64-apple-ios
2. 生成swift声明

   首先cd进入ffi文件夹，执行cargo build --release

   然后cd进入uniffi-bindgen文件夹，执行 cargo run --bin uniffi-bindgen generate --library ../ffi/target/release/libimffi.a --language swift --out-dir out
   可以看到out文件夹中生成了若干文件
3. 引入swift工程

   将步骤1生成的libimffi.a  步骤2生成的若干.h .swift文件引入swift工程 （demo中已引入相对文件夹位置）



#### 技术列表

语言： Rust

长链接协议： Mqtt

数据库类型： sqlite

服务端： mqtt 公共免费服务

主要涉及的三方库：

1. rumqttc   		mqtt 能力库
2. diesel	   		数据库能力库
3. r2d2                数据库连接池支持
4. thiserror		支持 error 类型转换
5. once_cell       单例模式支持
6. lazy_static      懒加载能力支持
7. serde              序列化与反序列化



实际项目中的项目可能非常复杂，对于数据库的设计也会因业务不同而复杂多样，同时会有重试能力、批量处理能力、收发确认机制等；

这里我们的目的是通过 项目实战去学习 Rust，所以只实现比较基础的收发以及数据库存储能力；

#### 简易架构图

![im](https://github.com/FaceWaller/blogImages/raw/master/readme/simpleim.png)



#### 项目地址

https://github.com/FaceWaller/IM_Client



源代码中包含两个demo，您可以同时运行它们，模拟两个客户端之间的互动。这有助于更直观地理解代码的工作原理。如果您觉得这对您有帮助，不妨点一颗小星星以表示支持！



