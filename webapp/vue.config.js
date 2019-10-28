module.exports = {
    outputDir: '../public',
    productionSourceMap: false,

    pwa: {
      name: 'Rust中文社区'
    },
    configureWebpack: {
      externals:{
        'vue': 'Vue',
        'vue-router': 'VueRouter'
      }
    }
}
