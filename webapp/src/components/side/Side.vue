<template>
    <div id="side">
        <main>
            <div id="title">
                <span style="font-size: 1.8rem;">Rust</span>
                <span style="font-size: 1.4rem;">中文</span>
            </div>
            <div id="line"></div>
            <p>支持Rust中文：详见
                <span style="font-weight:bold;"><a href="/a/office/theme/20" target="_blank"> 共建Rust中文宣告 </a></span></p>
            
            <p>本站<a href="https://github.com/rustlang-cn/ruster" target="_blank"><strong>源码</strong></a>开源.</p>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
export default {
  name: 'side'
}
</script>

<style scoped>
main{
    background-color: #ffffff;
    box-shadow: 0 0 3px rgba(0,0,0,0.1), 0 -1px 1px rgba(0,0,0,0.1);
}
main a {
    color: #7B463D;
}
main #title {
    padding: 0.5rem;
    color: #7B463D;
    text-align: center;
    font-weight:bold;
}
main #line {
    border: 1px solid #acc;
}
main p, main #title {
    margin: 0 auto;
    width: 10rem;
}
</style>
