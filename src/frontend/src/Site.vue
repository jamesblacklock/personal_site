<template>
	<div>
		<div
			class="dot"
			v-for="dot in dots"
			:key="dot.key"
			:style="{ width: dot.size, height: dot.size, left: dot.left, top: dot.top }">
		</div>
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
const dots = [];

for(let i=0; i<260; i++) {
	let size = Math.floor(Math.random() * 14);
	// size *= size;
	size = 1/size*5;
	dots.push({
		key: i,
		size: `${size}vw`,
		left: `${Math.floor(Math.random() * 100) - size}vw`,
		top: `${Math.floor(Math.random() * 100) - size}vh`,
	});
}

export default {
	name: 'Site',
	data: () => ({
		dots
	})
}
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
	position: relative;
	z-index: 1;
	min-height: 100vh;
	padding: 84px 36px 24px;
}
header {
	padding: 24px 36px;
	background: black;
	color: white;
	font-size: 24px;
	position: absolute;
	width: 100%;
	z-index: 2;
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
.dot {
	position: absolute;
	background: #f3f3f3;
	border-radius: 50%;
}
</style>
