import Vue from "vue";
import VueRouter from "vue-router";

import {getIdentity} from "../api/auth";

Vue.use(VueRouter);

const routes = [
    {
        path: "/login",
        name: "Login",
        component: () => import("../views/Login.vue"),
        meta: {guest: true}
    },
    {
        path: "/",
        name: "Hospital",
        component: () => import("../views/Hospital.vue"),
        meta: {admin: true}
    },
    {
        path: "/clinic/:id",
        name: "Clinic",
        component: () => import("../views/Clinic.vue"),
        meta: {admin: true}
    },
    {
        path: "/clinic",
        component: () => import("../views/Clinic.vue"),
        meta: {admin: true}
    },
    {
        path: "/specializations",
        name: "Specializations",
        component: () => import("../views/Specializations.vue"),
        meta: {admin: true}
    }
];

const router = new VueRouter({
    mode: "history",
    routes: routes,
});

router.beforeEach(async (to, from, next) => {
    if (to.matched.some(record => record.meta.admin)) {
        var identity = await getIdentity();
        if (identity && identity.role == 0) {
            next();
            return;
        }
        next("/login");
    } else {
        next();
    }
})

router.beforeEach(async (to, from, next) => {
    if (to.matched.some(record => record.meta.guest)) {
        var identity = await getIdentity();
        if (identity && identity.role == 0) {
            next("/");
            return;
        }
        next();
    } else {
        next();
    }
})



export default router;
