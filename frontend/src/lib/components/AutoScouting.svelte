<script lang="ts">
    import WhereAuto from "./WhereAuto.svelte";
    import NumberInput from "./NumberInput.svelte";
    import { auto_data } from "$lib/stores";
    import TextInput from "./TextInput.svelte";
    import { goto } from "$app/navigation";
    import Navbar from "./Navbar.svelte";
    export let scout_id = "";
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;
    let pressed = false;
    let teamNumber: Number;
    let locationkinda = "onstage";
    let timer = 0;
    let a: any = 'a';
    let b: any = 'b';
    let c: any = 'c';
    let d: any = 'd';
    let e: any = 'e';
    let f: any = 'f';
    let g: any = 'g';
    let h: any = 'h';
    let p: any = 'Preload';
    let matchkey = "";
    function onclick() {
        pressed = true;
        timer = 0;
    }

    function updateTimer() {
        timer += 0.01;
        if (Math.round(timer) == 15) {
            submit()
        }
    }
    function H() {
        h = Math.round(timer);
        h = h;
    }
    function P() {
        p = Math.round(timer);
        p = p;
    }
    function G() {
        g = Math.round(timer);
        g = g;
    }
    function F() {
        f = Math.round(timer);
        f = f;
    }
    function E() {
        e = Math.round(timer);
        e = e;
    }
    function D() {
        d = Math.round(timer);
        d = d;
    }
    function C() {
        c = Math.round(timer);
        c = c;
    }
    function B() {
        b = Math.round(timer);
        b = b;
    }
    function A() {
        a = Math.round(timer);
        a = a;
    }
    function submit() {
        if (a == 'a') {
            a = -1
        }
        if (b == 'b') {
            b = -1
        }
        if (c == 'c') {
            c = -1
        }
        if (d == 'd') {
            d = -1
        }
        if (e == 'e') {
            e = -1
        }
        if (f == 'f') {
            f = -1
        }
        if (g == 'g') {
            g = -1
        }
        if (h == 'h') {
            h = -1
        }
        if (p == 'Preload') {
            p = -1
        }
        $auto_data.a = a;
        $auto_data.b = b;
        $auto_data.c = c;
        $auto_data.d = d;
        $auto_data.e = e;
        $auto_data.f = f;
        $auto_data.g = g;
        $auto_data.h = h;
        $auto_data.p = p;
        $auto_data.id = 0;
        $auto_data.team_key = teamNumber.toString();
        $auto_data.match_key = "2024gal_qm" + matchkey;
        $auto_data.scout_id = scout_id;
        console.log($auto_data);
        fetch(`${BACKEND_URL}/submit/auto`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify($auto_data),
        });
        pressed = false
    }
    setInterval(updateTimer, 10);
</script>

{#if pressed == false}
    <h2>Auto Scouting</h2>
    <TextInput name="Match Number" bind:value={matchkey} />
    <NumberInput name="Team Number" bind:value={teamNumber} />
    <center><h4>What color is your robot, and which side of the field are you on?</h4></center>
    <WhereAuto bind:value={locationkinda} />
    <button class="realbutton" on:click={onclick}>Start Auto!</button>
    <footer class="bottom-div">
        <Navbar page="strategy"/>
    </footer>
{:else}
    <h2>
        {teamNumber}
    </h2>
    <h2>Timer: {Math.round(timer)}</h2>
    <button class="realbutton" on:click={onclick}>Reset Timer</button>
    <button style="background-color: orange;" class="realbutton" on:click={P}
        >{p}</button
    >
    
    <div style="" class="grid grid-cols-2 grid-rows-5 gap-1 hello">
        {#if locationkinda == "onstage"}
            <button style="background-color: orange;" on:click={H}>{h}</button>
            <button
                style="background-color: orange;"
                on:click={G}
                class="col-start-1 row-start-2">{g}</button
            >
            <button
                style="background-color: orange;"
                on:click={F}
                class="col-start-1 row-start-3">{f}</button
            >
            <button
                style="background-color: orange;"
                on:click={E}
                class="col-start-1 row-start-4">{e}</button
            >
            <button
                style="background-color: orange;"
                on:click={D}
                class="col-start-1 row-start-5">{d}</button
            >
            <button
                style="background-color: orange;"
                on:click={A}
                class="col-start-2 row-start-4">{a}</button
            >
            <button
                style="background-color: orange;"
                on:click={B}
                class="col-start-2 row-start-3">{b}</button
            >
            <button
                style="background-color: orange;"
                on:click={C}
                class="col-start-2 row-start-2">{c}</button
            >
        {:else if locationkinda == "parked"}
            <button
                style="background-color: orange;"
                on:click={H}
                class="col-start-2 row-start-1">{h}</button
            >
            <button
                style="background-color: orange;"
                on:click={G}
                class="col-start-2 row-start-2">{g}</button
            >
            <button
                style="background-color: orange;"
                on:click={F}
                class="col-start-2 row-start-3">{f}</button
            >
            <button
                style="background-color: orange;"
                on:click={E}
                class="col-start-2 row-start-4">{e}</button
            >
            <button
                style="background-color: orange;"
                on:click={D}
                class="col-start-2 row-start-5">{d}</button
            >
            <button
                style="background-color: orange;"
                on:click={A}
                class="col-start-1 row-start-4">{a}</button
            >
            <button
                style="background-color: orange;"
                on:click={B}
                class="col-start-1 row-start-3">{b}</button
            >
            <button
                style="background-color: orange;"
                on:click={C}
                class="col-start-1 row-start-2">{c}</button
            >
        {:else if locationkinda == "failed"}
            <button
                style="background-color: orange;"
                on:click={H}
                class="col-start-2 row-start-5">{h}</button
            >
            <button
                style="background-color: orange;"
                on:click={G}
                class="col-start-2 row-start-4">{g}</button
            >
            <button
                style="background-color: orange;"
                on:click={F}
                class="col-start-2 row-start-3">{f}</button
            >
            <button
                style="background-color: orange;"
                on:click={E}
                class="col-start-2 row-start-2">{e}</button
            >
            <button
                style="background-color: orange;"
                on:click={D}
                class="col-start-2 row-start-1">{d}</button
            >
            <button
                style="background-color: orange;"
                on:click={A}
                class="col-start-1 row-start-2">{a}</button
            >
            <button
                style="background-color: orange;"
                on:click={B}
                class="col-start-1 row-start-3">{b}</button
            >
            <button
                style="background-color: orange;"
                on:click={C}
                class="col-start-1 row-start-4">{c}</button
            >
        {:else if locationkinda == "notattempted"}
            <button
                style="background-color: orange;"
                on:click={H}
                class="col-start-1 row-start-5">{h}</button
            >
            <button
                style="background-color: orange;"
                on:click={G}
                class="col-start-1 row-start-4">{g}</button
            >
            <button
                style="background-color: orange;"
                on:click={F}
                class="col-start-1 row-start-3">{f}</button
            >
            <button
                style="background-color: orange;"
                on:click={E}
                class="col-start-1 row-start-2">{e}</button
            >
            <button
                style="background-color: orange;"
                on:click={D}
                class="col-start-1 row-start-1">{d}</button
            >
            <button
                style="background-color: orange;"
                on:click={A}
                class="col-start-2 row-start-2">{a}</button
            >
            <button
                style="background-color: orange;"
                on:click={B}
                class="col-start-2 row-start-3">{b}</button
            >
            <button
                style="background-color: orange;"
                on:click={C}
                class="col-start-2 row-start-4">{c}</button
            >
        {/if}
    </div>
    <button
        on:click={submit}
        style="background-color: #00D586;"
        class="realbutton">Submit!</button
    >
{/if}

<style>
    .bottom-div {
        position: fixed;
        bottom: 0;
        left: 0;
        width: 100%;
        background-color: #f0f0f0; /* Just for visualization */
        text-align: center;
    }
    h2 {
        @apply text-4xl;
        color: white;
        margin: 15px;
        font-family: Poppins-bold;
    }
    h3 {
        font-size: 30px;
        color: white;
        margin: 15px;
        font-family: Poppins-bold;
    }
    h4 {
        color: white;
        margin: 15px;
        font-family:Poppins-bold;
    }
    p {
        @apply font-medium text-xl;
    }
    button {
        font-family: "Poppins-Medium";
        margin: 15px;
        margin-top: 8px;
        margin-bottom: 8px;
        font-size: 30px;
        padding: 1rem;
        @apply text-text_white bg-btn_grey py-2 rounded-md;
    }
    .realbutton {
        width: calc(100% - 30px);
        font-size: 20px;
        padding: 0.3rem;
    }
    .hello {
        height: 60vh;
        margin-top: 25px;
        margin-bottom: 25px;
    }
</style>
