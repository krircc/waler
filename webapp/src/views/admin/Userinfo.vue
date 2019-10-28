<template>
    <div id="userinfo" v-if="signin_user != ''">
      <div id="show"><img src="../../../static/imgs/rust.png" /></div>
      <div id="title">
          <ul>
              <li><a >用户</a></li>
              <li><a >主题</a></li>
              <li><a >评论</a></li>
          </ul>
      </div>
      <main>
        <div id="container">
            <div id="center">
                <div id="items" v-for="(user, index) in user_list" :key="index">
                            <div id="item">
                                <div id="infos">
                                    <span id="info" :herf="'/a/user/' + user.id ">{{ user.id }}</span>&emsp;
                                    <span id="info">{{ user.created_at }} </span>
                                </div> 
                                <div id="item-title">
                                    <span id="info">{{ user.username }}</span>&emsp;
                                    <span id="info">{{ user.email }} </span>
                                </div>
                            </div>
                      </div>
            </div>
            <div id="aside">
            </div>
            <gotop></gotop>
        </div>
      </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import Gotop from '../../components/gotop/Gotop'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'userinfo',
  components: {
    "gotop": Gotop,
    "mnav": Mnav
  },
  data: function() {
    return {
      signin_user: '',
      user_list: ''
    }
  },
  mounted: function() {
      if (localStorage.getItem('token')){
            this.signin_user = JSON.parse(localStorage.getItem('signin_user'))
      }
      fetch(URLprefix + 'api/admin/users',{
                  method: 'GET',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                   this.user_list = json.admin_users.reverse()
              })
              .catch((e) => {
                console.log(e)
              })
  }
}
</script>

<style scoped>
#show {
    background-color: #f1a3d6;
}
#title {
    line-height: 3.3rem;
    background-color: #faeaf5;
}
#title #theme-title {
    padding-bottom: 0.2rem;
    border-bottom: 3px solid #a506a5;
}
#container a{
    color: #0541af;
}
#center {
    background-color: #ffffff;
}
#center #items #item {
    padding: 1.2vh 0.5vw;
    border-bottom: 1px solid #f3e1f8;
}
#center #items #item-title {
    margin-top: 1vh;
}
#center #items #infos {
    font-size: 0.85rem;
}
button, #aside #update input {
    width: 7rem; 
    line-height:25px;
    background-color:#ffffff;
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
  img {
      margin: 1vh 2vw;
      width: 5rem;
      height: 5rem;
      border-radius: 50%;
  }
  #title ul li {
      display: inline-block;
      padding-left: 3vw;
      font-weight: bold;
  }
  main{
      margin: 0 auto;
      width: 97%;
  }
  #container #aside {
      margin: 1vh auto;
      padding: 0.6rem;
      border: 1px solid rgb(212, 212, 212);
      background-color: #ffffff;
  }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    #show {
        padding-top: 5rem;
    }
    img {
        margin-left: 8vw;
        width: 7rem;
        height: 7rem;
        border-radius: 50%;
    }
    #title ul {
        margin-left: 6vw;
    }
    #title ul li {
        display: inline-block;
        font-weight: bold;
        padding-left: 2rem;
    }
    #title ul #item {
        padding-left: 2rem;
    }
    main {
        margin: 1rem auto;
        width: 80%;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 90%;
        margin-right: 1vw;
    }
    #container #aside #right{
        padding: 1rem;
        border: 1px solid rgb(212, 212, 212);
        background-color: #ffffff;
    }
    #container #aside #right #update {
        margin: 1rem auto;
    }
}
@media only screen and (min-width: 850px) {
    #show {
        padding-top: 6rem;
    }
    img {
        margin-left: 11vw;
        width: 8rem;
        height: 8rem;
        border-radius: 50%;
    }
    #title ul {
        margin-left: 10vw;
    }
    #title ul li{
        display: inline-block;
        font-weight: bold;
        font-size: 1.1rem;
        padding-left: 2rem;
    }
    main {
        margin: 1rem auto;
        width: 75%;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 1vw;
    }
    #container #aside {
        flex: 1;
    }

    #container #aside #right{
        padding: 1rem;
        border: 1px solid rgb(212, 212, 212);
        background-color: #ffffff;
    }
    #container #aside #right #update {
        margin: 1rem auto;
    }
    #center #items #item-title {
        font-size: 1.1rem;
    }
}
</style>