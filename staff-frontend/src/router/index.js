import Vue from "vue";
import VueRouter from "vue-router";

import { getIdentity } from "../api/auth";

Vue.use(VueRouter);

const routes = [
    {
        path: "/login",
        name: "Login",
        component: () => import("../views/Login.vue"),
        meta: { guest: true }
    },
    {
        path: "/",
        name: "Hospital",
        component: () => import("../views/Hospital.vue"),
        meta: { admin: true }
    },
    {
        path: "/clinic/:id",
        name: "Clinic",
        component: () => import("../views/Clinic.vue"),
        meta: { admin: true }
    },
    {
        path: "/clinics",
        name: "Clinics",
        component: () => import("../views/Clinic.vue"),
        meta: { admin: true }
    },
    {
        path: "/specializations",
        name: "Specializations",
        component: () => import("../views/Specializations.vue"),
        meta: { admin: true }
    },
    {
        path: "/provinces",
        name: "Provinces",
        component: () => import("../views/Provinces.vue"),
        meta: { admin: true }
    },
    {
        path: "/doctor/:id",
        name: "DoctorsInClinic",
        component: () => import("../views/Doctor.vue"),
        meta: { admin: true }
    },
    {
        path: "/doctors",
        name: "Doctors",
        component: () => import("../views/Doctor.vue"),
        meta: { admin: true }
    },
    {
        path: "/schedule/:id",
        name: "ScheduleInDoctors",
        component: () => import("../views/Schedule.vue"),
        meta: { admin: true }
    },
    {
        path: "/shift/:id",
        name: "ShiftsInDoctors",
        component: () => import("../views/Shift.vue"),
        meta: { admin: true }
    },
    {
        path: "/shifts",
        name: "Shifts",
        component: () => import("../views/Shift.vue"),
        meta: { admin: true }
    },
    {
        path: "/admins",
        name: "Admins",
        component: () => import("../views/AdminUser.vue"),
        meta: { admin: true }
    },
    {
        path: "/staffs",
        name: "Staffs",
        component: () => import("../views/StaffUser.vue"),
        meta: { admin: true }
    },
    //staf
    {
        path: "/staff/doctor",
        name: "StaffDoctors",
        component: () => import("../views/Doctor.vue"),
        meta: { staff: true }
    },
];

const router = new VueRouter({
    mode: "history",
    routes: routes,
});

router.beforeEach(async (to, from, next) => {
    var identity = await getIdentity();
    
    if (!identity) next("/login");
    
    if (to.matched.some(record => record.meta.admin)) {
        if (identity.role == 0) {
            next();
            return;
        }
        next("/login");
    } else if (to.matched.some(record => record.meta.staff)) {
        if (identity.role == 1) {
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
