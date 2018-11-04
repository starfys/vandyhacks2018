import Vue from "vue";
import Router from "vue-router";

import TaskList from "@/views/TaskList.vue";
import AddTask from "@/views/AddTask.vue";
import EditTask from "@/views/EditTask.vue";
import StopWork from "@/views/StopWork.vue";

Vue.use(Router);

export default new Router({
  mode: "history",
  base: process.env.BASE_URL,
  routes: [
    {
      path: "/",
      redirect: "/tasks",
    },
    {
      path: "/tasks",
      name: "taskList",
      component: TaskList
    },
    {
      path: "/tasks/add",
      name: "addTask",
      component: AddTask
    },
    {
      path: "/tasks/:id",
      name: "editTask",
      component: EditTask,
      children: [
        { path: "work", component: StopWork },
      ],
    },
    /*{
      path: "/analytics",
      name: "analytics",
      component: Analytics,
    },*/
    /*{
      path: "/about",
      name: "about",
      // route level code-splitting
      // this generates a separate chunk (about.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () =>
      // webpackChunkName: "about"
        import("./views/About.vue")
    },*/
  ]
});
