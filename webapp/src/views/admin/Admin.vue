<template>
    <div id="admin">
      <main>
        <div id="container">
            <div id="center">
                <div v-if="signin_user != ''">
                    <a href="/a/xyzruster/themeinfo">themeinfo</a><br>
                    <a href="/a/xyzruster/userinfo">userinfo</a><br>
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
    name: 'admin',
    components: {
        "gotop": Gotop,
        "mnav": Mnav
    },
    data: function() {
        return {
        signin_user: ''
        }
    },
    mounted: function() {
        if (localStorage.getItem('token')){
                let signin_user = JSON.parse(localStorage.getItem('signin_user'))
                this.signin_user = signin_user
        }
    },
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
    main {
        margin: 0 auto;
        width: 97%;
        padding-top: 1rem;
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    main {
        margin: 0 auto;
        width: 75%;
        padding-top: 6rem;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 1vw;
    }
  
}
@media only screen and (min-width: 850px) {
    main {
        margin: 0 auto;
        width: 72%;
        padding-top: 6rem;
    }
    #container {
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 1vw;
    }
}
</style>