module.exports = {
  devServer: {
    progress: false, // needed to prevent log-flood during dev ctr build
    disableHostCheck: true // needed for this -> https://stackoverflow.com/questions/51084089/vuejs-app-showing-invalid-host-header-error-loop
  },
  // pluginOptions: {
	// 	sitemap: {
	// 		urls: [
	// 		]
	// 	}
	// }
}