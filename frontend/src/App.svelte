<script lang="ts">
    import { Router } from "svelte-router-spa";
    import { user_id } from "./stores";
    //import Scout from "./pages/scout/Scout.svelte";
    //import Admin from "./pages/admin/Admin.svelte";
    //import Homepage from "./pages/homepage/Homepage.svelte";

    function userIsAdmin() {
        
    }

    function userIsAuthed() {
        
    }
    
    let routes = [
        { name: "/", component: Homepage },
        {
            name: "admin",
            component: Admin,
            onlyIf: { guard: userIsAdmin, redirect: "/" },
        },
        {
            name: "scout",
            component: Scout,
            onlyIf: { guard: userIsAuthed, redirect: "/" },
        },
    ];
</script>

<nav>
    <a href="https://accounts.google.com/o/oauth2/v2/auth?scope=openid%20profile%20email&client_id={oauth_id}&response_type=code&redirect_uri=http://localhost:8000/api/auth/google_callback\">
    Click here to sign into Google!
     </a>
</nav>

<Router {routes} />

