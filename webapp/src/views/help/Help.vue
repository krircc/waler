<template>
    <div id="help">
        <main>
            <div id="content">
                <div id="support">
                    <div id="ruster">
                        <span style="font-size: 1.6rem;">Rust</span>
                        <span style="font-size: 1.2rem;">中文 : 互助Rust爱好者,致力于Rust编程语言中文网络！</span>
                    </div><br>
                    <h3><a href="/a/office/theme/20" target="_blank">~共建Rust中文宣告~</a></h3>
                    <p>Rust中文 是一个社区项目，<a href="https://github.com/rustlang-cn/ruster" target="_blank">源码</a>&nbsp;每月需要费用花在云服务器托管，请帮助这个项目给予捐助，谢谢！
                    联系维护者krircc@aliyun.com.</p>
                </div><br>
                <div style="box-shadow: 0 0 3px rgba(0,0,0,0.1), 0 -1px 1px rgba(0,0,0,0.1);width:11.4rem;text-align:center;">
                    <div><img id="pay" src="../../../static/imgs/weixinpay.jpg"></div>
                    <strong>微信支付</strong>
                </div><br>
                <div style="box-shadow: 0 0 3px rgba(0,0,0,0.1), 0 -1px 1px rgba(0,0,0,0.1);width:11.4rem;text-align:center;">
                    <div><img id="pay" src="../../../static/imgs/alipay.jpeg"></div>
                    <strong>支付宝</strong>
                </div><br>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
export default {
    name: 'help'
}
</script>

<style scoped>
a {
  color: #0541af;
}
main #ruster {
    color: #7B463D;
    padding-top: 1rem;
    font-weight: bold;
}
main img {
    widows: 11rem;
    height: 11rem;
}
@media only screen and (max-width: 600px) {
    main  {
      margin: 0 auto;
      width: 90%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
}
@media only screen and (min-width: 850px) {
    main {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
}
</style>
