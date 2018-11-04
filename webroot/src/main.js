import '@babel/polyfill';
import Vue from "vue";
import './plugins/bootstrap-vue';
import 'vue-awesome/icons';
import Icon from 'vue-awesome/components/Icon';
import "vue-datetime/dist/vue-datetime.css";
import Datetime from "vue-datetime";

import App from "./App.vue";
import router from "./router";
import "./registerServiceWorker";
import store from './store';


Vue.config.productionTip = false;

Vue.component('v-icon', Icon);
Vue.use(Datetime);

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount("#app");
