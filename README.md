# FlyMall商城

#### 介绍
FlyMall Rust版，是基于actix-web和rbatis的商城管理系统，一款面向全球的多语言、多业态电子商务平台，系统提供管理、支付、物流、售后等众多应用功能，满足跨境市场的基本需 求，助力更多商户走出国门，大力开拓国外市场，为广大用户提供一站式的国际化电商运营解决方案

###### 已完成功能
1. 管理员登录
2. 菜单管理
3. 角色管理
4. 部门管理
5. 文章管理

### 软件架构
前台为了SEO采用tera模板引擎渲染，管理后台采用Vue+ElementUI

##### 后台技术选型

| 说明      | 框架        | 说明   |                  |
|---------|-----------|------|------------------|
| 基础框架    | actix_web | 模板引擎 | tera             |
| 持久框架    | Rbatis    | 程序构建 | Cargo            |
| 关系型数据库  | MySQL     | 权限验证 | actix-web-grants | 
| 缓存      | Redis     | 搜索查询 | Elasticsearch    |
| 负载均衡    | Nginx     | 定时任务 | job_scheduler    |
| 短信      | 阿里云短信     | 认证   | Jsonwebtoken     |
| 日志处理    | Log4rs    | 接口规范 | RESTful          |


#### 安装教程

1.  xxxx
2.  xxxx
3.  xxxx

#### 使用说明

1.  xxxx
2.  xxxx
3.  xxxx

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 特技

1.  使用 Readme\_XXX.md 来支持不同的语言，例如 Readme\_en.md, Readme\_zh.md
2.  Gitee 官方博客 [blog.gitee.com](https://blog.gitee.com)
3.  你可以 [https://gitee.com/explore](https://gitee.com/explore) 这个地址来了解 Gitee 上的优秀开源项目
4.  [GVP](https://gitee.com/gvp) 全称是 Gitee 最有价值开源项目，是综合评定出的优秀开源项目
5.  Gitee 官方提供的使用手册 [https://gitee.com/help](https://gitee.com/help)
6.  Gitee 封面人物是一档用来展示 Gitee 会员风采的栏目 [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)
