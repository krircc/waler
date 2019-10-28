export default {
    user: {
		authenticated: false,
		role: null
    }, 
    checkAuth() {
		var jwt = localStorage.getItem('token');
		var role = localStorage.getItem('signin_user');
		if (jwt) {
			this.user.authenticated = true;
			this.user.role = role;
		} else {
			this.user.authenticated = false;
		}
    },

    getAuthHeader () {
        return {
            headers: {
                'Authorization': 'Bearer ' + localStorage.getItem('token')
            }
        }
    }
}
