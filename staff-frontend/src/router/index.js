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
        meta: { auth: true }
    },
    {
        path: "/clinic/:id",
        name: "Clinic",
        component: () => import("../views/Clinic.vue"),
        meta: { auth: true }
    },
    {
        path: "/clinics",
        name: "Clinics",
        component: () => import("../views/Clinic.vue"),
        meta: { auth: true }
    },
    {
        path: "/specializations",
        name: "Specializations",
        component: () => import("../views/Specializations.vue"),
        meta: { auth: true }
    },
    {
        path: "/provinces",
        name: "Provinces",
        component: () => import("../views/Provinces.vue"),
        meta: { auth: true }
    },
    {
        path: "/doctor/:id",
        name: "DoctorsInClinic",
        component: () => import("../views/Doctor.vue"),
        meta: { auth: true }
    },
    {
        path: "/doctors",
        name: "Doctors",
        component: () => import("../views/Doctor.vue"),
        meta: { auth: true }
    },
    {
        path: "/schedule/:id",
        name: "ScheduleInDoctors",
        component: () => import("../views/Schedule.vue"),
        meta: { auth: true }
    },
    {
        path: "/shift/:id",
        name: "ShiftsInDoctors",
        component: () => import("../views/Shift.vue"),
        meta: { auth: true }
    },
    {
        path: "/shifts",
        name: "Shifts",
        component: () => import("../views/Shift.vue"),
        meta: { auth: true }
    },
    {
        path: "/admins",
        name: "Admins",
        component: () => import("../views/AdminUser.vue"),
        meta: { auth: true }
    },
    {
        path: "/staff",
        name: "Staff",
        component: () => import("../views/StaffUser.vue"),
        meta: { auth: true }
    },
];

const router = new VueRouter({
    mode: "history",
    routes: routes,
});

router.beforeEach(async (to, from, next) => {

    if (to.matched.some(record => record.meta.auth)) {
        var identity = await getIdentity();                        
        if (identity) {
            next();
            return;
        }
        next("/login");
    } else {
        next();
    }

});

router.beforeEach(async (to, from, next) => {
    if (to.matched.some(record => record.meta.guest)) {
        var identity = await getIdentity();
        if (identity) {
            if(identity.role == 0) {
                next("/");
                return;
            } else if (identity.role == 1) {
                next(`/doctor/${identity.clinic["$oid"]}`);
                return;
            }
        } 
        next();
    } else {
        next();
    }
});



export default router;
