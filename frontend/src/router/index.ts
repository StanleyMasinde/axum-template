import { createRouter, createWebHistory } from "vue-router";

import HomePage from "../pages/HomePage.vue";
import AnotherPage from "../pages/AnotherPage.vue";
import NotFoundPage from "../pages/NotFoundPage.vue";
import HealthCheck from "@/pages/HealthCheck.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomePage,
    },
    {
      path: "/another-page",
      name: "another-page",
      component: AnotherPage,
    },
    {
      path: "/health-check",
      name: "health-check",
      component: HealthCheck
    },
    {
      path: "/:pathMatch(.*)*",
      name: "not-found",
      component: NotFoundPage,
    },
  ],
});

export default router;
