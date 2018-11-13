import Vue from "vue";
import Router from "vue-router";

import TaskList from "@/views/TaskList.vue";
import AddTask from "@/views/AddTask.vue";
import EditTask from "@/views/EditTask.vue";
import StopWork from "@/views/StopWork.vue";
import Analytics from "@/views/Analytics.vue";
import PredictionsAnalytics from "@/views/PredictionsAnalytics.vue";
import ProductivityAnalytics from "@/views/ProductivityAnalytics.vue";
import MeetingsAnalytics from "@/views/MeetingsAnalytics.vue";

Vue.use(Router);

export default new Router({
  mode: "history",
  base: process.env.BASE_URL,
  routes: [
    {
      path: "/analytics",
      name: "analytics",
      component: Analytics,
      children: [
        { path: "predictions", component: PredictionsAnalytics },
        { path: "productivity", component: ProductivityAnalytics },
        { path: "meetings", component: MeetingsAnalytics },
      ],
    },
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
    },
    {
      path: "/tasks/:id/work",
      name: "finishWork",
      component: StopWork,
      /*children: [
        { path: "questions/music",
          component: MusicQuestion,
          name: "musicQuestion", },
        { path: "questions/interruptions",
          component: InterruptionsQuestion,
          name: "interruptionsQuestion", },
        { path: "questions/noise",
          component: NoiseQuestion,
          name: "noiseQuestion", },
        { path: "questions/meetings",
          component: MeetingsQuestion,
          name: "meetingsQuestion", },
        { path: "questions/breaks",
          component: BreaksQuestion,
          name: "breaksQuestion", },
      ],*/
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
