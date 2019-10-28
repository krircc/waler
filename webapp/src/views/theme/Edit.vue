<template>
    <div id="edit">
        <main>
            <div id="container">
                <div id="center">
                    <div id="new-title"><p>更新主题</p></div>
                    <form id="form" >
                            <div id="topic-group">
                                <input type="text" name="title" v-model="Title" placeholder="Please input title">
                            </div>    
                            <div id="editor">
                                <mavon-editor name="content" v-model="Content" :ishljs = "true" style="height: 100%;" :toolbars="set"></mavon-editor>
                            </div>
                            <div id="new">
                                    <button type="submit" id="submit" @click="post" >发布</button>
                            </div>
                    </form>
                </div>
               <side></side>
            </div>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
import { mavonEditor } from 'mavon-editor'
import 'mavon-editor/dist/css/index.css'
import URLprefix from '../../config'
import Side from '../../components/side/Side'
export default {
    name: 'edit',
    components: {
        mavonEditor,
        "side": Side
    },
    data () {
        return {
            Title: '',
            Content: '',
            set:{
                bold: true, // 粗体
                italic: true, // 斜体
                header: true, // 标题
                quote: true, // 引用
                ul: true, // 无序列表
                ol: true, // 有序列表
                link: true, // 链接
                code: true, // code
                table: true, // 表格
                trash: true, // 清空
                fullscreen: true, // 全屏编辑
                htmlcode: true, // 展示html源码
                preview: true, // 预览
                help: true, // 帮助
                
                underline: false, // 下划线
                strikethrough: false, // 中划线
                mark: false, // 标记
                alignleft: false, // 左对齐
                aligncenter: false, // 居中
                alignright: false, // 右对齐
                superscript: false, // 上角标
                subscript: false, // 下角标
                undo: false, // 上一步
                redo: false, // 下一步
                imagelink: false, // 图片链接
                readmodel: false, // 沉浸式阅读
                save: false, // 保存（触发events中的save事件）
                navigation: false, // 导航目录
                subfield: false, // 单双栏模式
            }
        }
    },
    mounted: function() {
        fetch(URLprefix + 'api/theme/'+ this.$route.params.id,{
            method: 'GET',
        }).then(response => response.json())
        .then(json => {
            this.Title = json.theme.title
            this.Content = json.theme_raw_content
        }).catch((e) => {
            console.log(e)
        })
    },
    methods: {
        post() {
            if (localStorage.getItem('signin_user')){
                this.signin_user = JSON.parse(localStorage.getItem('signin_user'))
                let theme_id = Number.parseInt(this.$route.params.id)
                let category_name = this.$route.params.category
                let title = this.Title
                let content = this.Content
                let user_id = JSON.parse(localStorage.getItem('signin_user')).id
                if(title == ''){
                    alert("标题不能为空")
                    return
                }else if(content == ''){
                    alert("内容不能为空")
                    return
                }else{
                    let data = { 
                        theme_id: theme_id,
                        user_id: user_id,
                        category_name: category_name,
                        title: title,
                        content: content
                    }
                    fetch(URLprefix + 'api/theme_new', {
                        body: JSON.stringify(data), 
                        headers: {
                            'content-type': 'application/json'
                        },
                        method: 'POST',
                    }).then(response => response.json())
                    .then(json => {
                        // return
                        this.$router.push('/')
                    })
                    .catch((e) => {
                        console.log(e)
                    })
                    this.$router.push('/')
                }
            }else{
                alert("非法用户, 请先登陆再操作")
                return
            }
        }
    }
}
</script>

<style scoped>
#center {
  background-color: #FFFFFF;
}
#center #new-title {
    width: 100%;
    line-height: 33px;
    border :1px solid #CAC1C1;
    background-color:#f5fdfa;
}
#center form #topic-group input {
   margin: 10px auto;
   height: 30px;
   width: 100%;
}
#center #editor {
    margin: auto;
}
#center form #new button {
    margin-top: 0.3rem;
    width:63px; 
    line-height:25px;
    background-color:#FFFFFF;
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
    main{
        margin: 2vh auto;
        width: 97%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 90%;
        margin-right: 1vw;
    }
    #container #side {
        flex: 1;
    }
}
@media only screen and (min-width: 850px) {
    main {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 1vw;
    }
    #container #side {
        flex: 1;
    }
}
</style>