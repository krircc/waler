<template>
  <div id="homepage">
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
                              <div id="item">
                                <div id="item-title">
                                  <span id="category-name">{{ theme.category_name_cn }}</span><span><a :href="'/a/'+ theme.category_name + '/theme/' + theme.id" title="theme.title">{{ theme.title }}</a></span>
                                </div>
                                <div id="detail">
                                    <!-- <div id="info"><a :href="'/a/user/' + theme.user_id">{{ theme.username }}</a></div> -->
                                    <div class="info" id="comment"><a :href="'/a/'+ theme.category_name + '/theme/' + theme.id">{{ theme.comment_count }}</a></div>
                                    <div class="info" id="view">{{ theme.view_count }}</div>
                                    <div class="info" id="time">{{ theme.rtime }}</div>
                                    <!-- <div id="more">  ••  </div> -->
                                </div> 
                              </div>                           
                            </div>
                          </div>
                      </div>
              </div>
              <div v-if="$route.params.number <= page_count">
                      <ul id="pagination">
                            <li id="one" > <a href="/">1</a></li>

                            <li v-if="$route.params.number <= 2"></li>
                            <li v-else><a :href="'/a/home/page/' + ($route.params.number - 1)"> << </a></li>
                            
                            <li >••</li>

                            <li v-if="(half_count - 3) > 2" ><a :href="'/a/home/page/' + (half_count - 3)">{{ half_count - 3 }}</a></li>
                            <li v-if="half_count > 2" ><a :href="'/a/home/page/' + half_count" >{{ half_count }}</a></li>
                            <li v-if="(half_count + 3) < page_count" ><a :href="'/a/home/page/' + (half_count + 3)" >{{ half_count + 3 }}</a></li>

                            <li >••</li>

                            <li v-if="$route.params.number == page_count"></li>
                            <li v-else><a :href="'/a/home/page/' + ($route.params.number - (-1))">>></a></li>
                            
                            <li ><a :href="'/a/home/page/' + page_count">{{ page_count }}</a></li>  
                        </ul>       
              </div>
              <div v-else>
                  <ul id="pagination">
                      <li id="one" > <a href="/">1</a></li>
                  </ul>  
              </div>
          </div>
          <side></side>
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
  name: 'homepage',
  components: {
    "gotop": Gotop,
    "side": Side
  },
  data: function() {
    return {
      theme_list: '',
      page_count: '',
      half_count:'',
      categorys: ''
    }
  },
  mounted: function() {
      let home_page_id = this.$route.params.number
      let page_id = Number.parseInt(home_page_id)
      let data = { page_id: page_id, }
             fetch(URLprefix + 'api/theme_list/page_id',{
                 body: JSON.stringify(data), 
                 headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                    json.theme_list.map((item) => {
                      if (item.user_avatar == "") {
                        item.user_avatar = "https://www.gravatar.com/avatar/1"
                      }
                    })
                    this.theme_list = json.theme_list
                    this.page_count = json.theme_page_count
                    this.half_count = Math.ceil(json.theme_page_count/2)
                    this.categorys = json.categorys
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
#center #items #line {
    display: flex;
    padding: 0 0.4rem;
    border-bottom: 1px solid #f3e1f8;
}
#center #theme_item {
    flex: 1;
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
#center #items #item #item-title a {
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
    #center #items #item #item-title {
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
    #center #avatar img {
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
    #container #side {
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
    #center #items #item {
        display: flex;
    }
    #center #items #item #item-title {
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
    #container #side {
        flex: 1;
    }
}
        
</style>