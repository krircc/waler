<template>
  <div id="home">
      <main>
        <div id="container">
          <div id="center">
              <div id="header">
                <li  ><a href="/a/home/best" >最美</a></li>
                <li  ><a href="/" >全部</a></li>
                <span v-for="(category, index) in categorys" :key="index">
                    <li v-if="category.category_name != 'office'">
                      <a :href="'/a/home/' + category.category_name" >{{ category.category_name_cn }}</a>
                    </li>
                </span>
                <li  ><a href="/a/home/care" >未回复</a></li>
              </div>
              <div id="content">
                      <div id="items" v-for="(theme, index) in theme_list" :key="index">
                          <div id="line">
                            <di id="avatar">
                              <a :href="'/a/user/' + theme.user_id"><img :src= theme.user_avatar ></a>
                            </di>
                            <div id="theme_item">  
                              <div id="office" v-if="theme.category_name === 'office'">
                                  <div id="office-title">
                                    <span id="category-name">{{ theme.category_name_cn }}</span><span><a :href="'/a/'+ theme.category_name + '/theme/' + theme.id" title="theme.title">{{ theme.title }}</a></span>
                                  </div>
                                  <div id="detail">
                                      <!-- <span><a :href="'/a/user/' + theme.user_id">{{ theme.username }}</a></span> -->
                                      <div class="info" id="comment"><a :href="'/a/'+ theme.category_name + '/theme/' + theme.id">{{ theme.comment_count }}</a></div>
                                      <div class="info" id="view">{{ theme.view_count }}</div>
                                      <div class="info" id="time">{{ theme.rtime }}</div>
                                  </div> 
                              </div>
                              <div id="item" v-if="theme.category_name !== 'office'">
                                  <div id="item-title">
                                    <span id="category-name">{{ theme.category_name_cn }}</span><span><a :href="'/a/'+ theme.category_name + '/theme/' + theme.id" title="theme.title">{{ theme.title }}</a></span>
                                  </div>
                                  <div id="detail">
                                      <!-- <div><a :href="'/a/user/' + theme.user_id">{{ theme.username }}</a></div> -->
                                      <div class="info" id="comment"><a :href="'/a/'+ theme.category_name + '/theme/' + theme.id">{{ theme.comment_count }}</a></div>
                                      <div class="info" id="view">{{ theme.view_count }}</div>
                                      <div class="info" id="time">{{ theme.rtime }}</div>
                                  </div>
                              </div>
                            </div>
                          </div>
                      </div>
              </div>
              <div >
                      <ul id="pagination">
                            <li id="one" > <a href="/">1</a></li>
                            <li v-if="page_count > 2"> <a href="/a/home/page/2">2</a></li>

                            <li >••</li>

                            <li v-if="(half_count - 3) > 2" ><a :href="'/a/home/page/' + (half_count - 3)">{{ half_count - 3 }}</a></li>
                            <li v-if="half_count > 2" ><a :href="'/a/home/page/' + half_count" >{{ half_count }}</a></li>
                            <li v-if="(half_count + 3) < page_count" ><a :href="'/a/home/page/' + (half_count + 3)" >{{ half_count + 3 }}</a></li>

                            <li >••</li>

                            <li ><a :href="'/a/home/page/' + page_count">{{ page_count }}</a></li>  
                        </ul>       
              </div>
          </div>
          <div id="rightside">
              <div id="bestside">
                  <div id="show">
                    <h3>最美的人</h3>
                    <div id="title">
                      <li><h5>最近最美</h5></li><li>&emsp;<strong>|</strong>&emsp;</li><li><h5>一直最美</h5></li>
                    </div>
                    <ul>
                      <div id="bestperson" v-for="(new_person,index) in new_best" :key="index">
                        <li><a :href="'/a/user/' + new_person.id">{{new_person.username}}</a></li><li>&emsp;<strong>|</strong>&emsp;</li><li><a :href="'/a/user/' + all_best[index].id">{{all_best[index].username}}</a></li>
                      </div>
                    </ul>
                </div>
              </div>
              <div id="rusterinfo">
                <h3>统计信息</h3>
                  <li>会员:{{ruster_info[0]}}</li>
                  <li>主题:{{ruster_info[1]}}</li>
                  <li>博客:{{ruster_info[2]}}</li>
                  <li>评论:{{ruster_info[3]}}</li>
                  <li>收藏:{{ruster_info[4]}}</li>
              </div>
              <side></side>
          </div>
          <gotop></gotop>
        </div>
      </main>
  </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import Side from '../../components/side/Side'
import Gotop from '../../components/gotop/Gotop'
export default {
  name: 'home',
  components: {
    "side": Side,
    "gotop": Gotop
  },
  data: function() {
    return {
      theme_list: '',
      page_count: '',
      categorys: '',
      half_count:'',
      new_best: '',
      all_best: '',
      ruster_info: ''
    }
  },
  mounted: function() {
             let data = { page_id: 1}
             fetch(URLprefix + 'api/theme_list/page_id',{
                 body: JSON.stringify(data), 
                 headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.page_count = json.theme_page_count
                  this.half_count = Math.ceil(json.theme_page_count/2)
                  json.theme_list.map((item) => {
                    if (item.user_avatar == "") {
                      item.user_avatar = "https://www.gravatar.com/avatar/1"
                    }
                  })
                  this.theme_list = json.theme_list
                  this.categorys = json.categorys
              })
              .catch((e) => {
                console.log(e)
              })  

              fetch(URLprefix + "api/home/bestperson",{
                  method: 'GET',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.new_best = json.new_best
                  this.all_best = json.all_best
              })
              .catch((e) => {
                console.log(e)
              })    

              fetch(URLprefix + 'api/ruster/info',{
                  method: 'GET',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  json.ruster_info[0] = json.ruster_info[0]
                   this.ruster_info = json.ruster_info
              })
              .catch((e) => {
                console.log(e)
              })
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#center {
  background-color: #FFFFFF;
}
#header {
  padding: 0.8rem 0.4rem;
  box-shadow: 0 0 3px rgba(0,0,0,0.1), 0 -1px 1px rgba(0,0,0,0.1);
}
#header li {
  display: inline-block;
  color: #7B463D;
  font-weight: bold;
  margin-right: 1rem;
}
#container #center #items #line {
    display: flex;
    padding: 0 0.4rem;
    border-bottom: 1px solid #f3e1f8;
}
#container #center #theme_item {
    flex: 1;
}
#center #office #office-title {
    color: #7B463D;
    font-weight: bold;
}
#center #item #item-title a:visited {
    color: gray;
}
#center #theme_item #category-name {
    color: #7B463D;
    font-size: 0.8rem;
    background-color: limegreen;
    padding: 0.2rem 0.3rem;
    border-radius: 11%;
}
#center #theme_item #office-title #category-name {
    color: #FFFFFF;
    background-color: purple;
}
#center #items #office #office-title a, #center #items #item #item-title a {
    padding-left: 0.4rem;
}
#center #pagination li {
  display: inline; 
  border: 1px solid #cfd9ee;
  padding: 0.33rem;
  vertical-align: middle;
  line-height: 2.2rem;
}
#center #pagination #one{
  border: 1px solid #5bb383;
  margin-left: 0.4vw;
}
#center #pagination a{
  color: #0541af;
  font-weight: bold;
}
#container #rightside h3 {
    padding: 0.5rem;
    color: #7B463D;
    text-align: center;
    border-bottom: 2px solid #acc;
}
#container #rightside #bestside #show #title, #container #rightside #bestside #show ul {
    line-height: 2rem;
    text-align: center;
}
#container #rightside #bestside #show #title {
    border-bottom: 2px solid rgb(206, 209, 209);
}
#container #rightside #bestside #show li {
    display: inline-block;
}
#container #rightside #bestside #show ul li {
    font-size: 1.1rem;
    color: #7B463D;
}
#bestside, #rusterinfo {
  padding: 0.1rem;
  background-color: #FFFFFF;
  box-shadow: 0 0 3px rgba(0,0,0,0.1), 0 -1px 1px rgba(0,0,0,0.1);
  margin-bottom: 0.5rem;
}
#container #rightside #rusterinfo  {
  text-align: center;
}
#container #rightside #rusterinfo h3 {
  border-bottom: 2px solid #acc;
}
@media only screen and (max-width: 599px) {
    main{
        margin: 1vh auto;
        width: 97%;
    }
    #header {
        padding: 0.3rem;
    }
    #container #center #avatar img {
        width: 2.5rem;
        height: 2.5rem;
        margin-top: 0.4rem;
        border-radius: 50%;
    }
    #center #items #office #office-title, #center #items #item #item-title {
        padding: 0.4rem 0 0 0.4rem;
    }
    #center #items #detail {
        padding: 0 0 0.2rem 0.5rem;
    }
    #center #items #detail .info {
        display: inline-block;
        padding-right: 0.8rem;
        font-size: 0.77rem;
        color: #7B463D;
    }
    #container #rightside{
        margin-top: 1rem;
    }
}
@media only screen and (min-width: 600px) and (max-width: 999px) {
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
    #container #center #avatar img {
        width: 2.5rem;
        height: 2.5rem;
        margin-top: 0.4rem;
        border-radius: 50%;
    }
    #center #items #office #office-title, #center #items #item #item-title {
        padding: 0.4rem 0 0 0.4rem;
    }
    #center #items #detail {
        padding: 0 0 0.2rem 0.5rem;
    }
    #center #items #detail .info {
        display: inline-block;
        padding-right: 0.8rem;
        font-size: 0.77rem;
        color: #7B463D;
    }
    #container #rightside {
        flex: 1;
    }
}
@media only screen and (min-width: 1000px) {
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
    #container #center #avatar img {
        width: 2.4rem;
        height: 2.4rem;
        margin-top: 0.3rem;
        border-radius: 50%;
    }
    #center #items #office, #center #items #item {
        display: flex;
    }
    #center #items #office #office-title, #center #items #item #item-title {
        width: 80%;
        padding: 0.6rem 0.4rem 0.4rem;
    }
    #center #items #detail {
        flex: 1;
        display: flex;
        padding-top: 1rem;
        color:gray;
        font-size: 0.88rem;
    }
    #center #items #detail #comment {
        text-align: center;
        flex-basis: 2rem;
    }
    #center #items #detail #view {
        text-align: center;
        flex-basis: 3rem;
    }
    #center #items #detail #time {
        text-align: center;
        flex-grow:1;
    }
    #container #rightside {
        flex: 1;
    }
}
</style>