<template>
    <div id="gotop">
        <main>
            <button @click="topFunction" id="myBtn" title="Go to top">Top</button>
        </main>
    </div>
</template>

<script>
/* eslint-disable */
export default {
  name: 'gotop',
  mounted: function() {
      window.addEventListener('scroll',function(){
        var scrollTop = document.documentElement.scrollTop || document.body.scrollTop;
        if(scrollTop>20){
            document.getElementById("myBtn").style.display = "block";
        }else{
            document.getElementById("myBtn").style.display = "none";
        }
      },true)
  },
  methods: {
      topFunction() {
        // When the user clicks on the button, scroll to the top of the document
        document.body.scrollTop = 0; // For Chrome, Safari and Opera 
        document.documentElement.scrollTop = 0; // For IE and Firefox
      }
  }
}
</script>

<style scoped>
main #myBtn {
        display: none; /* Hidden by default */
        position: fixed; /* Fixed/sticky position */
        z-index: 99; /* Make sure it does not overlap */
        border: none; /* Remove borders */
        outline: none; /* Remove outline */
        background-color: #7B463D; /* Set a background color */
        font-weight: bold;
        color: white; /* Text color */
        cursor: pointer; /* Add a mouse pointer on hover */
        padding: 10px; /* Some padding */
        border-radius: 5px; /* Rounded corners */
}
@media only screen and (max-width: 600px) {
    main #myBtn {
        bottom: 1rem; /* Place the button at the bottom of the page */
        right: 5%; /* Place the button 30px from the right */
    }
}
@media only screen and (min-width: 600px) and (max-width: 850px) {
    main #myBtn {
        bottom: 1rem; /* Place the button at the bottom of the page */
        right: 11%; /* Place the button 30px from the right */
    }
}
@media only screen and (min-width: 850px) {
    main #myBtn {
        bottom: 1rem; /* Place the button at the bottom of the page */
        right: 18%; /* Place the button 30px from the right */
    }
}
</style>