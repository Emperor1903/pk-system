import Vue from 'vue'
import VueRouter from 'vue-router'

Vue.use(VueRouter)

const routes = [
<<<<<<< HEAD
    {
        path: '/',
        name: 'Home',
        component: () => import("../views/Home.vue"),
    },
]

const router = new VueRouter({
    mode: "history",
    routes: routes
=======
  {
    path: '/',
    name: 'Home',
    component: Home
  }, {
    path: '/search',
    name: 'Search',
    component: () => import('../views/Search')
  },
  {
    path: '/about',
    name: 'About',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/About.vue')
  }
]

const router = new VueRouter({
  mode: "history",
  routes: routes
>>>>>>> b8a70a69d7fd114e1bfcad8b202e6e22aca78198
})

export default router;
