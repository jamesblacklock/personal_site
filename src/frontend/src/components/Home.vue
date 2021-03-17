<template>
	<div>
		<img style="float:left;width:200px;margin-right:20px" :src="require('../assets/me.jpg')">
		<p>
			This is my personal site. I don't do social media, but I have a
			<a href="http://github.com/jamesblacklock/">github</a>. The only thing interesting
			on there at the time of writing is <a href="http://github.com/jamesblacklock/caesar">
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
/* h3 {
	margin: 40px 0 0;
}
ul {
	list-style-type: none;
	padding: 0;
}
li {
	display: inline-block;
	margin: 0 10px;
} */
</style>
