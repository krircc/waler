<template>
  <div id="signadmin">
      <div id="content">
          <div id="title">    
            <router-link to="/a/signin">登陆</router-link>&emsp;|&emsp;
            <router-link to="/a/signup">注册</router-link> 
          </div>
            <input type="text" name="username" placeholder="用户名" v-model="Username" />
            <input type="password" name="password" placeholder="密码" v-model="Password" />
            <input type="text" name="code" placeholder="代码" v-model="Acode" />
          <div>
              <div id="v_container" style="height: 44px;"></div>
              <input type="text" id="code_input" value="" placeholder="请输入上方验证码" style="width: 80%;"/>
              <span><button id="verify" >验证</button></span>
          </div>
          <button id="submit" @click="signin">登陆</button>
      </div>
  </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import  '../../../static/js/gVerify.js'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'signadmin',
  components: {
    "mnav": Mnav
  },
  data () {
    return {
      Username: '',
      Password: '',
      Acode: ''
    }
  },
  mounted: function() {
    var verifyCode = new GVerify("v_container");
    document.getElementById("verify").onclick = function () {
      var res = verifyCode.validate(document.getElementById("code_input").value);
      if (res) {
        let verify = document.getElementById("verify")
        verify.innerHTML = "成功"
      } else {
        let verify = document.getElementById("verify")
        verify.innerHTML = "失败"
      }
    }
  },
  methods: {
    signin () {
      let uname = this.Username
      let password = this.Password
      let acode = this.Acode;
      let data = { 
          username: uname,
          password: password,
          code: acode,
      }
      if (document.getElementById("verify").innerHTML == "成功") {
          if (acode.length == ''){
              alert("代码不能为空.")
              return
          }else{
            fetch(URLprefix + 'user/signin', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                    if (json.status == 200) {
                        localStorage.setItem('token',json.token);
                        localStorage.setItem('signin_user',JSON.stringify(json.signin_user));
                        window.location.reload ( true ); 
                        this.$router.push('/a/xyzruster/allinfo')
                    }else{
                        alert(json.message)
                    }
              })
              .catch((e) => {
                console.log(e)
              })
          }
      }else{
          alert("请先成功通过验证码再登陆.")
      }
              
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#content {
    width: 18rem;
    margin: 0 auto;
    padding-top: 33px;
}
#title {
    text-align: center;
    padding: 0.5rem 0;
    font-size: 1.3rem;
    font-weight: bold;
    background-color:bisque;
}
input[type="text"],
input[type="password"] {
  margin: 6px auto auto;
  width: 18rem;
  height: 36px;
  border: none;
  border-bottom: 1px solid #AAA;
  font-size: 16px;
}
#verify {
  width: 20%; 
  padding: 6px 0;
  font-size: 1rem;
  background-color: bisque;
  border: none;
}
#submit  {
  margin: 6px 0;
  width: 18rem;
  height: 40px;
  background-color:rgb(250, 212, 165);
  border: none;
  border-radius: 2px;
  font-weight: bold;
  font-size: 1.1rem;
}
@media only screen and (min-width: 600px) {
    #content {
      margin: 0 auto;
      padding-top: 100px;
    }
}
</style>