<script lang="ts">
	import { page } from '$app/stores';
</script>

<header>
	<div class="corner">
	</div>

	<nav>
		<svg viewBox="0 0 2 3" aria-hidden="true">
			<path d="M0,0 L1,2 C1.5,3 1.5,3 2,3 L2,0 Z" />
		</svg>
		<ul>
			<li><a class:active={$page.url.pathname === '/'} sveltekit:prefetch href="/">Steps</a></li>
			<li>
				<a class:active={$page.url.pathname === '/flows'} sveltekit:prefetch href="/flows">Flows</a>
			</li>
			<li>
				<a class:active={$page.url.pathname === '/stats'} sveltekit:prefetch href="/stats">Stats</a>
			</li>
		</ul>
		<svg viewBox="0 0 2 3" aria-hidden="true">
			<path d="M0,0 L0,3 C0.5,3 0.5,3 1,2 L2,0 Z" />
		</svg>
	</nav>

	<div class="corner">
	</div>
</header>

<style>
	header {
		display: flex;
		justify-content: space-between;
	}

	.corner {
		width: 3em;
		height: 3em;
	}

	nav {
        --custom-red: #A7233A;
        --custom-blue-flat: #297EA6;
        --custom-blue-bright: #0066FF;
        --inactive-color: rgb(255, 255, 255, 0.4);
        --active-color: #ffffff;
		display: flex;
		justify-content: center;
		--background: var(--custom-blue-bright);
	}

	svg {
		width: 2em;
		height: 3em;
		display: block;
	}

	path {
		fill: var(--background);
	}

	ul {
		position: relative;
		padding: 0;
		margin: 0;
		height: 3em;
		display: flex;
		justify-content: center;
		align-items: center;
		list-style: none;
		background: var(--background);
		background-size: contain;
	}

	li {
		position: relative;
		height: 100%;
	}

	nav a {
		display: flex;
		height: 100%;
		align-items: center;
		padding: 0 1em;
		color: var(--inactive-color);
		font-weight: 600;
		font-size: 0.8rem;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		text-decoration: none;
		transition: color 0.2s linear;
	}

    nav a.active {
        color: var(--active-color);
    }

	a:hover {
		color: var(--active-color);
	}
</style>
