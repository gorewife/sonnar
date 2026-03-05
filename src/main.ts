import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";

import { library } from "@fortawesome/fontawesome-svg-core";
import { faPlus, faGear, faFolder } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { useUIStore } from "./stores/UIStore";

library.add(faPlus, faGear, faFolder);

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);

const ui = useUIStore();
ui.initTheme();

app.component("font-awesome-icon", FontAwesomeIcon);
app.mount("#app");
