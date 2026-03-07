<script>
    import { onMount } from "svelte";
    import { socket } from "shared/socket";
    import { initial } from "shared/match-data.svelte";


    let data = $state(initial());
    let websocket;

    onMount(() => {
        websocket = new WebSocket("ws://localhost:9001");

        websocket.onmessage = (message) => {
            Object.assign(data, JSON.parse(message.data));
        };
    })
</script>

<main class="container flexc">
    <div class="player-info">
        <span class="info">{data.player1.name}</span>
    </div>
    <div class="versus flexc">
        <h1>VS</h1>
    </div>
    <div class="player-info">
        <span class="info">{data.player2.name}</span>
    </div>
</main>

<style>
    .flexc {
        display: flex;
        justify-content: center;
        align-items: center;
    }
    .container {
       background-color: #444;
       padding: 0.8rem;
    }
    .player-info {
        padding: 0 1rem;
        color: whitesmoke;
    }
    .info {
        font-size: 2rem;
    }
    .versus {
        padding: 1rem;
        background-color: whitesmoke;
        color: #444;
        width: 5rem;
        border-radius: 100%;
    }
</style>