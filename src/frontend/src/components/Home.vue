<template>
	<div>
		<div class="me"></div>
		<p>
			This is my personal site. I don't do social media, but I have a
			<a href="http://github.com/jamesblacklock/" target="_blank">github</a>. The only thing interesting
			on there at the time of writing is <a href="http://github.com/jamesblacklock/caesar" target="_blank">
			Caesar</a>, a programming language I've created.
		</p>
	</div>
</template>

<script>
export default {
	name: 'Home',
	props: {
		msg: String
	}
}
</script>

<style scoped>
.me {
	float: left;
	width: 200px;
	height: 240px;
	margin-right: 20px;
	background: url('../assets/me.jpg');
	background-size: cover;
	background-position: center;
}
</style>
