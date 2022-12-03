/*
    scijava
    Copyright (C) 2022 Kamila Szewczyk

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::cmp::max;
use std::f64::consts;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JObject, JValue};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jlong, jint, jobject};
use rug::Float;
use rug::float::{Constant, Special};
use rug::ops::Pow;

use crate::scifloat_elementary::xlat_rounding;

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_rf(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, n: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let n = n as *mut Float;
    let a = unsafe { &*a };
    let n = unsafe { &*n };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    // gamma(a + n) / gamma(a)
    let ga = dest.clone().gamma();
    *dest += n;
    dest.gamma_mut();
    *dest /= ga;
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_ff(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, n: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let n = n as *mut Float;
    let a = unsafe { &*a };
    let n = unsafe { &*n };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    // gamma(a + 1) / gamma(a - n + 1)
    let mut ga = dest.clone();
    ga += 1;
    ga.gamma_mut();
    *dest -= n;
    *dest += 1;
    dest.gamma_mut();
    *dest /= ga;
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_beta(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, b: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let b = b as *mut Float;
    let a = unsafe { &*a };
    let b = unsafe { &*b };
    let dest = unsafe { &mut *dest };
    let mut bp = b.clone();
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    if b.prec() != precision as u32 {
        bp.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    let mut apb = dest.clone() + &bp;
    dest.gamma_mut();
    bp.gamma_mut();
    apb.gamma_mut();
    *dest *= bp;
    *dest /= apb;
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sech(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.cosh_mut();
    dest.recip_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_csch(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.sinh_mut();
    dest.recip_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_coth(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.tanh_mut();
    dest.recip_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_rgamma(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.gamma_mut();
    dest.recip_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_loggamma(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.ln_gamma_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sinpi(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    *dest *= Float::with_val(precision as u32, Constant::Pi);
    dest.sin_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_cospi(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    *dest *= Float::with_val(precision as u32, Constant::Pi);
    dest.cos_mut();
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sinc(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    dest.sin_mut();
    *dest /= a;
}

fn lambertw_special(prec: u32, z: Float, k: f32) -> Float {
    if z.is_zero() {
        if k == 0.0 {
            return z;
        }
        return Float::with_val(prec, Special::NegInfinity);
    }
    if z.is_infinite() && k == 0.0 {
        return z;
    }
    z.ln()
}

fn lambertw_approx_hybrid(prec: u32, z: Float, k: f32) -> Float {
    if k == 0.0 {
        if z < -4.0 && z < 4.0 && -1.0 < z && z < 2.5 {
            // Taylor series near -1
            if z < -0.5 {
                return Float::with_val(prec, Special::Nan);
            }
            // return real type
            let r = Float::with_val(prec, -0.367879441171442);
            let mut z = z;
            if z > r {
                z = r.clone()
            }
            // Singularity near -1/e
            if z < -0.2 {
                let zmr = z - r;
                let zmr_r = zmr.clone().sqrt();
                return Float::with_val(prec, -1) + Float::with_val(prec, 2.33164398159712) * zmr_r - Float::with_val(prec, 1.81218788563936) * zmr;
            }
            // Taylor series near 0
            if z < 0.5 {
                return z;
            }
            // Simple linear approximation
            return Float::with_val(prec, 0.2) + Float::with_val(prec, 0.3) * z;
        }
        
        let l1 = z.ln();
        let l2 = l1.clone().ln();
        let a = l1.clone() - &l2;
        let b = l2.clone() / &l1;
        return a + b + l2.clone() * (l2 - 2) / (2 * l1.pow(2));
    } else if k == -1.0 {
        // return real type
        let r = Float::with_val(prec, -0.367879441171442);
        let mut z = z;
        if r < z && z < 0.0 {
            z = r.clone();
        }
        if z < 0.1 && -0.6 < z && z < -0.2 {
            let zmr = z - r;
            let zmr_r = zmr.clone().sqrt();
            return Float::with_val(prec, -1) - Float::with_val(prec, 2.33164398159712) * zmr_r - Float::with_val(prec, 1.81218788563936) * zmr;
        } else if (-0.2..0.0).contains(&z) {
            let l1 = (-z).ln();
            let ml1 = -l1.clone();
            return l1 - ml1.ln();
        } else {
            return Float::with_val(prec, Special::Nan);
        }
    }
    Float::with_val(prec, Special::Nan)
}

// return the logarithmic magnitude of a number, i.e. m such that |x| <= 2^m. m can be zero or infinity.
fn mag(x: &Float) -> f64 {
    if x.is_zero() {
        return f64::NEG_INFINITY;
    }
    if x.is_infinite() {
        return f64::INFINITY;
    }
    if x.is_sign_negative() {
        let x = -x.clone();
        let mut m = x.log2();
        m.floor_mut();
        -m.to_f64()
    } else {
        let mut m = x.clone().log2();
        m.floor_mut();
        m.to_f64()
    }
}

fn lambertw_series(prec: u32, z: Float, k: f32, tol: Float) -> (Float, bool) {
    let magz = mag(&z);
    if (-10.0..900.0).contains(&magz) && (-1000.0..1000.0).contains(&k) {
        // Near the branch point at -1/e
        if magz < 1.0 && (0.36787944117144..0.41787944117144).contains(&(z.clone() + 0.36787944117144)) {
            if k == 0.0 || k == -1.0 {
                let delta = Float::with_val(prec, -1).exp() + &z;
                let cancellation = -mag(&delta);
                let fc: Float = Float::with_val(prec, consts::E) * z + 1.0;
                let mut p = Float::with_val(prec + cancellation as u32, 2.0) * fc.sqrt();
                let mut u = vec![Float::with_val(prec, -1.0), Float::with_val(prec, 1.0)];
                let mut a = vec![Float::with_val(prec, 2.0), Float::with_val(prec, -1.0)];
                if k != 0.0 {
                    p = -p;
                }
                let mut s = Float::with_val(prec + cancellation as u32 / 2, 0.0);
                // The series converges, so we could use it directly, but unless
                // *extremely* close, it is better to just use the first few
                // terms to get a good approximation for the iteration
                for l in 2..max(2, cancellation as usize) {
                    if u.len() <= l {
                        a.push(Float::with_val(prec, 0.0));
                        for j in 0..l {
                            a[l] += u[j].clone() * &u[l + 1 - j];
                        }

                        u.push(Float::with_val(prec, 0.0));
                        u[l] = (l - 1) * (u[l - 2].clone() / 2.0 + a[l - 2].clone() / 4.0) / (l + 1) - a[l].clone() / 2.0 - u[l - 1].clone() / (l + 1);
                    }
                    let term = u[l].clone() * p.clone().pow(l as u32);
                    s += &term;
                    if mag(&term) < -tol.clone() {
                        return (s, true);
                    }
                }
                return (s, false);
            }
            if k == 0.0 || k == -1.0 {
                return (lambertw_approx_hybrid(prec, z, k), false);
            }
        }
        if k == 0.0 {
            if magz < -1.0 {
                return (z.clone() * (1.0 - z), false);
            }
            let L1 = z.ln();
            let L2 = L1.clone().ln();
            return (L1.clone() - &L2 + L2.clone() / &L1 + L2.clone() * (L2 - 2.0) / (2.0 * L1.pow(2)), false);
        } else if k == -1.0 && (-0.36787944117144..0.0).contains(&z) {
            let L1 = (-z).ln();
            return (L1.clone() - (-L1).ln(), false);
        }
    }
    (Float::with_val(prec, Special::Nan), false)
}

fn lambertw(env: JNIEnv, prec: u32, z: Float, k: f32) -> Option<Float> {
    if !z.is_normal() {
        return Some(lambertw_special(prec, z, k));
    }
    let wp = prec + 30;
    let tol = prec - 5;
    let (mut w, done) = lambertw_series(wp, z.clone(), k, Float::with_val(wp, 10.0).pow(tol).recip());
    if !done {
        // Use Halley iteration to solve w*exp(w) = z
        let two = Float::with_val(wp, 2.0);
        let mut converged = false;
        for _ in 0..100 {
            let ew = w.clone().exp();
            let wew = w.clone() * &ew;
            let wewz = wew.clone() - &z;
            let wn = w.clone() - &wewz / (wew.clone() + &ew - (w.clone() + &two) * &wewz / (two.clone() * &w + &two));
            if mag(&(wn.clone() - &w)) <= mag(&wn) - tol as f64 {
                w = wn;
                converged = true;
                break;
            } else {
                w = wn;
            }
        }
        if !converged {
            // throw exception
            let _ = env.throw(("java/lang/ArithmeticException", "Lambert W iteration failed to converge."));
            return None;
        }
    }
    Some(w)
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_lambertw(
        env: JNIEnv, _class: JClass, prec: u32, z: jlong, k: jint) -> jobject {
    let z = unsafe { &*(z as *const Float) };
    let lw = lambertw(env, prec, z.clone(), k as f32);
    let lw = match lw {
        Some(lw) => lw,
        None => return JObject::null().into_raw(),
    };
    let lw = Box::into_raw(Box::new(lw));
    let lw = lw as jlong;
    let class = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[JValue::Long(lw)]);
    match class {
        Ok(class) => class.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/ArithmeticException", "Lambert W iteration failed to converge."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_j0(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.j0_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.j0_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_j1(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.j1_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.j1_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_jn(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, n: jint) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.jn_mut(n);
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.jn_mut(n);
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_li2(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.li2_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.li2_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_gamma(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.gamma_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.gamma_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_gamma_inc(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong, x: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let x = x as *mut Float;
    let a = unsafe { &*a };
    let x = unsafe { &*x };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.gamma_inc_mut(x);
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.gamma_inc_mut(x);
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_erf(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.erf_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.erf_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_erfc(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.erfc_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.erfc_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sinh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.sinh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.sinh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_cosh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.cosh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.cosh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_tanh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.tanh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.tanh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_asinh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.asinh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.asinh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_acosh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.acosh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.acosh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_atanh(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.atanh_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.atanh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sin(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.sin_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.sin_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_cos(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.cos_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.cos_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_tan(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.tan_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.tan_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_sec(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.sec_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.sec_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_csc(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.csc_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.csc_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_cot(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.cot_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.cot_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_asin(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.asin_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.asin_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_asin_inplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.asin_mut();
    } else {
        a.set_prec(precision as u32);
        a.asin_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_acos(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.acos_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.acos_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_acos_inplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.acos_mut();
    } else {
        a.set_prec(precision as u32);
        a.acos_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_atan(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.atan_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.atan_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_atan_inplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.atan_mut();
    } else {
        a.set_prec(precision as u32);
        a.atan_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_asinh_inplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.asinh_mut();
    } else {
        a.set_prec(precision as u32);
        a.asinh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_acosh_inplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.acosh_mut();
    } else {
        a.set_prec(precision as u32);
        a.acosh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_atanh_inplace(
        _env: JNIEnv, _class: JClass, precision: jint, _rounding_mode: jint, a: jlong) {
    let a = a as *mut Float;
    let a = unsafe { &mut *a };
    if a.prec() == precision as u32 {
        a.atanh_mut();
    } else {
        a.set_prec(precision as u32);
        a.atanh_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_digamma(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.digamma_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.digamma_mut();
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_ei(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    if a.prec() == precision as u32 {
        *dest = a.clone();
        dest.eint_mut();
    } else {
        *dest = a.clone();
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
        dest.eint_mut();
    }
}

static GLAISHER: &str = "1.2824271291006226368753425688697917277676889273250011920637400217404063088588264611297364919582023743942064612039900074893315779136277528040415907257386172752214334327143439787335067915257366856907876561146686449997784962754518174312394652761282138081802192645168515461439199010835737307035049038881234188136749781330509377083368222249411587483734806439997883007012556700128699415770543205392758540581731588155481762970384743250467775147374600031616023046613296342991558095879293363438872887019889534607252331847024890010917769417121535691936749672612703980135265266886897821889740172937584075016747211489528881599666874316451389030696264559870469543740253099606800842447417554061490189444139386196089129682173528798629884342203669899006069808887858495874940853073471170901326675675033105234052210541417677615630819191999718523704776131231537413530472581981479745176102754083494314384965234139453373065832325673954957601692256427736926358821692159870775858274695751628415506485858908341282275562095470029185932630793733769420775222909401870869519573780711309667351770300199761916284102623752726816378229033734362580494428680534032732042900846388391121443268645907695322159861366344442033554934595473821711591745604101002930492625112760511436168822617838706520052547696311207973657035728266384457899280631694242451954988151325366692161252001708106116018610671004232418417513377404348117699563780827214950026507397969144159508267914013816355892647950012294804770964816714462947135859883101363461756945149272023528328959303815923955421876019291625627383231584901693310392651848996797903214816910231549698036932109117295224884743050550252809497007322567477110396595417135548692845597929341098737537652512512661225403654880032266002933262537959163473468271758618169150469200740838114315860319533176038617126076122986827366410238048487279321731735446291673144740735575957693075255002776287275812097190100801787362118912389920254966310233030979061883957501760893187795073235489593623591038338476240918953300311231689539454187331216482896439653125520938777622545329416060171577126740091702695712401567939659824865868334811407981497586407944583164343236586341469243684488038140655789889513418850023410808969669015900110470070709506055838180181565299949993820286544476219155333509838424110514821311309172195748203988082887828563480173388752722640657163675797028348836977718554576610580743607728143515605132173318974986613101000652042759295748672035186481615537220614228303754992800767761832967069463698507746144385837887735783033217527213604023404944977017229046097812588273004847206131000807141179783234977566412902188675235980281085252019061739473573684486719087068832369865559523031503501423546067845824334059495775673632277826179049394955234282200538510061277435645586944876969607587096975202950789923928326031141121543285468224705608031018659715705980943900639531185873385004129131295139738632550668277617634605380568203273377682923304983704205184179510104438729423780292479053983990300691669020699932491037230479259546257181493518832592665361540986888808041361470764400979608746599288393953819553278470476843040775312690334562270051872129866986534551273628204080578397438553981609261151015083614617024147954823938198723690420482681178363576464502761852776945871305844288815442894058065285363064586585212545708329078014216573401492717672393296653351402302350810254204181167959119645265740940764367581199887745218173336071392558034870323027076815258734319422854043571406865581672870752023566617949039270375435174860449517463105839624898470670231541078662430707648693698426875208703585266176589078259072921099414435701977651719385187068734936166787386022517116684661634793106266873616576818426113243529663177215401477379847546007576666798967792377403041487449627678698234280819482736430292457527860914858685992262263786631204209777122522947415874265664316926508059058135201466644620501555701556071740102354731004531373379200352223299792959812101342561805847044502390454247790268996589093640136692243570271115257103114933209912085473729717377787520566912244712933306292535351001532597344403508220902435980834425236045062571475833926871402866777669017816505146415834798545383292317016057495488025278356939049957937123239594588080783584962218590292637466475463244436870963976676397331412908152455142112872208164102670108911851465639153981090774818870610784378593994735606021879760029060095477214603935862273163558190611000010801892313868720549578252485373064021527308240590274854914249997924433774335730227323545179345952144683223183033944156021037569259409406305888256282163861557087652059830825635482427965822637294650769342939240485086284764124797962759808587412656670386343511016821017736845765408568450354753117012110933727170539045283211806814015969764589527195040911736358274409102422973676865350075831041070188659797627706748122250187334624892358214549411661597139198522020813691177451300316573225625527824160586138875816517885608804277716274963068165343262399902181142234766836960138455718907026077081745953812833949060708621577618763787141329285212742841056084413266428171784761709432017490848888346016172497320391243172627997606229163672337736809332663051937013173782063334676053457826847345476692895382129055605056290310281006180029267386185716632840844454423714734756587480741141843203657397130901941736859829103622610077387184462259690657602189998396202487565903243769619676275735319894594680708443520489664376848824412816121609372306327832232138151290125925044997443576657204108744702422169208235271272807978565085717116617692439927739390466754517163824372158030130566718755104415177874987942505913129648219945958700269832094678374591040314159660832897368809732533289507336756854412423802517615598105223590434499986238688510934181835625574620704618238021724275205435897940755881168259752774663515065651488538815521071138483561788560814797743372937976197479109985934557712975079759623150645881381159272309471885141025840397724590849919016399302320636894224668856885157600583406076930247738492346739333198177447662919530388470098135672558229841742733698312989691489829308958279037818525925114374286259029658817139148079836804872270824844920332208828320130295406524992420821320496070320947220301926535271812318310326023142666339599974404501775706794452330467846772366472138555080232888328321728703463466504868154485881853360513853960314387382794324813407552468499665195296973766239702431938866158595750219649406750378437201554295594704758959829173028025618944498846971047442460415298151224320448829442972054766020566316981130371208145071983416548062573273794391983854830344064033350162254761235456088176437627703186818854248925701560640428125598541679525233538572644525616703587677325423668603695085859352816956816791731607668631321770025255703816109646007211317552603297541442709503107758825340423863104893230039202077965257587580765921341717881830074485893466732720319072256531652695642508661144274390728120727055818370348974084389861334188054488560827903156107510619863012921908979411172170828410086977318104359610006921223100894881953353844088682687035340744420411568264104345759681892782940584325391172124004419014406393087670343925342638720551760718031021096266665474678929878140940571075878782885522826596899492517353509448058504927312386758940677862149696633157327310466355959500861115177116948905987680612057117168780320806583271707453411330189106412129069328036822451659290855180972950696832119470144959018255846225137503957936960989957125049779656117539893680852561260402010963899356286755388080627042381406381253428798811985063954297884626067185932579201731360839834519359374637462514624443446468470441524776512700293921418027761035333141100262692276745178153986117121682287688628543300334158274997553393221542063576198902589171211234602165948504588540216127403360697663551576150652360049636960708619877768584634506975811952610520063115030612894051869197609108731768361846046906175568026931142951016537488821192903398940454504623719901301597096485946062825672051398917227911026510836088969549259442987370717751800958727043347076617162068206351171195810438704596656616243306612374730921277532429625135133126228826883514433796228356807110331661937875691067892375333983746120018943763969645793837490706216507350395021583237659079353968827871174155597889747896610387993884887683017103578789637398452777013823587330331202332207726785047139494938111971244989276850907027037136688256526910372041126838173680479019214118749442706659502847376706193681785744555919314824030038073096458121474222067775280304389450278869839370729320521445582507663951625346049083042847856584255936628462409779728331064576376426629077344123407761385499914929360910569097848071542515467195677565775325708748546249390967361431700047505654373726880015614074636089345881669945710668866323553621954681088104081644742830145253999543733926309768010334724571912330533152061145918086924686904174138251776966078122495253015081430978808556167005217815342780927881893455351867219372539868861807178941077194940231777253772079871385364011401405958341069660947075542753386140036388744704373203728941932082481780544006647041931947113813695159876834656555760620782018534055148440530439469906206833802590448534757826127911648919887595922253767932542715254725823810169996672476651815464965119948022122628598131799906327833715668354378413542987807071346089570973174342473793650842984461357704571272985465579640572073111134255851814954964491976817795732287392627391361327638142375937710194735837066566771776936812901833849110971614574777497858023569674148814674925189783074773797886463736307840361589802612329279116489178492636284863215918023517040856205573232445368778569387602017962591812961010089641218356142084972282888031380917190360486078321879982614880637375497817480514809565660261199171713715111793721342282744604182802125292000559012481409380599293333246633457323932352598812652061609674825629698813965686969458561528337843819757669417973457435895987479951180947206272135710509955517358788693939328536144215346485349914417542027162353346354450021400157666591908397304759034271021855642154914109576801206015044872115271683371810877794946624203472218210674542085499228698870654129905805489485693100725371346591660657426139563924121265629259644101719430079756364479794573988775328357434181633122216154703611094289668229543931970902864799957234417624033247873204711768049752118711955579428132018218637775639782606335325655314591951784393303583754770093047552749268524930375777958197708510564464724016723645110036812631624329341390211741671884603690344947896381248183200638035244670522845542477563419132859696416363498384745209025542466111795900519155887535635588405596705429179305556801865802412025814242930848866653774924731142638312699855556608114430161040227300664221222003661102618559767434298926378401213933813917817987148535352261441017238707974143278190013855731826582786815132770748718901484235005749558870215663056020775665450833770523628136775487502801282109848192596940255771745323925469706483517298712560018248954633465121904668292399381369982505102051320338730066165985933316905766861331301346197543614750577947517475212996978820201691096672538206173806270715472751765543690606219573403465071682438010637532098869201369993186576071274172166642185817403594967193580478359352390975890389573696660719482659343590452679151841880137985245024701530393827476563274433200108254077709135511052703521261727369964587323409177155749346214443096453450590976642893124658229472815889156294394732198926769867067938866292138500693417586845452441935747013579276050588843363755217132688071953369747719079225122915444014117860690809646660440154761410473396172552472794627337309807457076358161236604774334023658091068743538708700091575627987033771262731626928786187955810882024500622980023600239783710200965768418067585519718496674624576566835167383347828405964054068383452465450489036382097542421150544291458894373767243024337910359609450461569329114925791719263595622413290265591495511787259008925763299103092230203230923849947259220001885137872139283086942199745416559353414514886799514534679082662748848297277224225720322718734139546653110571193877636614868436819358368238389249334241428506880150000355933132056182904390535832279879696446214026912859949316155236323485841520873179905171735718052720740978615611725513924683433793012911080575327135101615703961982865026431973666708263154132673917468718693261202965960765383573095449266741915657183304294440419118480127704294640014836367591541429082212273090729984880552849630211802796190749207439597029942656346205858743013075065413467967003982588317656840223366943369782967191953371602852974227414306819229014215499474585443900114403023948551149016698293694258187140186991267462923573992231923699639037212634160710940127334442114739709128985198575754612674980114757145678389228602042195370037968968222290723238444133734259168927145982405406267071353266217380818454601087676380410538410079182409917332003922320120754499929464260047039174372500630730114346908317975391474520820634933255343486339947464375690230914433757742971001833194268915267774751926376819836079872900264427157302449698825603084096858293484240548534410468783298048768252303084902428564423713702011197720250260970491895868877363894819878475710506989623816971879975659898709554124593540173000621503948962419791783201398944553456724447231937432910202466199427319147729788787315143023523284664436520665592632402512059239568238241563148815647728527486628209529586084512092672755307909734578987364636761599638665485760882966301111762895456405201661896493277485699652684799534543832308190607044615102361548143306152874792627700133362279537088383759769897147647569309793782968773908984516949017016419443787596354272766267367263851463104980047808357119424480330181844985741844311496008661065934578500391270370513315040370138402416113237866292878544554732898183487541703809508555664125528837599627774674421562635720706399662946846465682903898568302613799005010353733110045155144768557743108466366919799572837323680006443805637153365408343547595979857932975359459551884583728990990413622295888543342920466221794486771250576160586220746460198714138403964710436928595633238980361420433946325273645284126459746365444729007455436563208464706386567679482267900755393189109518231516785942532984883501439242035622578191230313482849711061242295124364811475591088330931697909331032435335832135123580986207340938703647433167380075768131198642715623509361149537988207369360313996876349048467987516648155475580591094593351771990449484712331785787947291439214879685539644384832601671504112689862653095080851236077419413142757512029347566664033303249750801477083053893521471560514857206727658000706281797693481806287972174692435666652075407918008837529198079362723909944555598414164714271104387989504760955907986276888512525590813457900193696539165701464551348867133790336035433430808213260329238604529116556608759319226217986121829162334529753083724306095857752386214152131144653421596831433363201218317569298797306882794474957545500127699341655240562527974474741506547844230438884169652470618920103574915170963076187076229485721822138578633208020616138230457829641898577989500859987987160559198760940889653261492448551056872220874165271394607455989821763649996238352148854438286484480437522626197123651456997269469735528665683542330364412448285513071954935040769148950220204917951912776081644347891121468162706428946963596400132796449866993585121640388166260257427184492021549803257290583481413525846117566502359754855222266138964626399620721899939125251003725560496957123209645513945825458463699451481317186491903338323584302908832425454290312861746480836359131148294245755832817009540856421670789905963487413326470598308492956439326309414164726248077022911489171590780008589024429247525723220801062118432207351676110610488857085716399274051676660514010045102030582138477474332224487499667178411009860448054134774007256200669935145524335587536841535164640785573469367035882129177687538913725066729167438951775216305272284178377093191980940834020070558052327395430464378290363253256150470867545913015727076798782066581617624505967415793953720705446673693547660893705733268042559385968646918811809872935376881415433265938307756808689027007219006097606736484942333758793861241693314547678323627391785453248124832419824269947132568522615883741149700603160785186288046479784610902325841859733996020509478980749370010877304638104440926407813249945306375271253813249982094801817793803804959497987129773659456400019441819204849845359922696346828177590132830580511299427880878421229601203422432687130218845867690381146053673495190835273941884758614803117309942519855668316429248291793833801395479788636181360803005808651888301142899052519063127073304897154596575123872236683096379336200783465137406705359935365069627370705426321901068433773674311234305717282030953954915686702190786452716084953077447884026456817041409087088058348816579911292219830616291405410261069129723786893309278801357846152558182233322044617139601989180071785156830656413683325645647139882922468513365495766805656085826030921298080599738246411352217651119403133111210436731160788635720487750982930195745030575867880200917611347098051592463292696388901091664448496754091795771317687686255065217810870304034447171033928554894147860287633183988047027294790415291742077714826881487785639821509584419576704032941457295037281925246974649156075249103454962191224091271788947971758413570722493137142164981756859958998241100960267704010365569077094563231348740088318153762515880822342622609983123868456249639285778173448010841977425858444072564343781227154120498528920000126363898787691703271227823663116525744663256167566619569199272428650185136048492734778045254132017243019652113485503088421801706406337469600763562807623448454162602919069761659439311498246464481997925580273877969938165786511110756618527601125065544924459145641440824540288161144107533632627863595951702744452027027722066560419379087106553709643849193944137353365754678208823438646108504480498710922834094113337992232689250427077927904614533875290188248069962941349705571211966070109611190693357846368151195350117644291893225320607490405057933370933745865044123693490854099643773148081789749194433819620717610646745862947743086797208643451588909222367304912261295552351941323564512925988610307288945615320940886956892087105162724447207616339398489462368872456818596304072070119574904902457688573010103769531994665073152293199935033049925253665081125992025409425835227538487607211602440452373959406838383335060201071331009244616724024040801331025943297621518331725270151801155980748632821020437390034907290113808466123603560854882289857339899346001039357434791308880476446487005550211457329194531386196066544500703323077829465979536228850526037365971128352929950795641600398521950007156757596245396380094843103582338270936805192392371998127028576649669031112977512765738605608730921250227024542727945496415128906066262130244043447878915043569422824947468181466488733001703939964891567808627084688716193627101136509902396650006438151732439237499338269608893088681421254395154167254945606610785310966757312581212854833412392031601084066593593665958358359459455128796605360131932757470020175241478173076989414957250666010836820071278297743213205585354549567697898208282434326955033803146906892644376611200949419131553123916901713486903818481049548494428060591243899992165058260592150343556668353511731315617499447347445368028425033906914757141330383434878902132712288454258924358003052432804401859874487545071548536216817158923690603105987994802307365898779221519629370221715799951657612991564669784368842477244940616408335184502205314766867759337012117558361446479395684484023249675150387347885922068086338646966045873531948869496684347943953487464233097730968956398787485198140498206809942207003210230423720958864327615658378636236322343345857958662249575331014012332869466881178978000036921911643906124933357728750106215772432439637167884224559591841662941631342143920184065332156626249382912125548777823941813390251";

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_glaisher(
        env: JNIEnv, _class: JClass, precision: jint) -> jobject {
    let n = Float::with_val(precision as u32, Float::parse(GLAISHER).unwrap());
    let ptr = Box::into_raw(Box::new(n));
    let ptr = ptr as jlong;
    let obj = env.new_object("palaiologos/scijava/SciFloat", "(J)V", &[ptr.into()]);
    match obj {
        Ok(obj) => obj.into_raw(),
        Err(_) => {
            let _ = env.throw(("java/lang/RuntimeException", "Failed to allocate object."));
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_palaiologos_scijava_SciFloat_harmonic(
        _env: JNIEnv, _class: JClass, precision: jint, rounding_mode: jint, dest: jlong, a: jlong) {
    let dest = dest as *mut Float;
    let a = a as *mut Float;
    let a = unsafe { &*a };
    let dest = unsafe { &mut *dest };
    *dest = a.clone();
    if a.prec() != precision as u32 {
        dest.set_prec_round(precision as u32, xlat_rounding(rounding_mode));
    }
    // H_n = eulergamma + digamma(n + 1)
    *dest += 1;
    dest.digamma_mut();
    *dest += Float::with_val(precision as u32, Constant::Euler);
}
