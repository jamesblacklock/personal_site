<template>
	<div>
		<header>
			<router-link id="site-name" to="/">
				james blacklock
			</router-link>
		</header>
		<div id="content">
			<router-view></router-view>
		</div>
	</div>
</template>

<script>
export default { name: 'Site' }
</script>

<style>
* {
	box-sizing: border-box;
}
#site {
	font-family: Avenir, Helvetica, Arial, sans-serif;
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
	color: #2c3e50;
}
#content {
	min-height: 100vh;
	padding: 84px 36px 24px;
	background: url('assets/background.png');
}
header {
	padding: 24px 36px;
	background: black;
	color: white;
	font-size: 24px;
	position: absolute;
	width: 100%;
}
#site-name {
	color: white;
	text-decoration: none;
	border: 1px solid white;
	padding: 1px 8px;
	border-top-color: transparent;
	border-right-color: transparent;
}
#site-name a:visited {
	color: white;
}
#site-name a:hover {
	color: #e9e9e9;
}
a {
	color: #df9037;
}
a:visited {
	color: #ce6824;
}
</style>
