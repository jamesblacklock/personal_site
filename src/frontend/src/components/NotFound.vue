<template>
	<div>
		<p>invalid URL!</p>
		<p><router-link to="/">let's go back home</router-link></p>
	</div>
</template>

<script>
export default { name: 'NotFound' }
</script>

<style scoped>
	* {
		text-align: center;
		font-size: 24px;
	}
	div {
		margin-top: 20vh;
	}
</style>
