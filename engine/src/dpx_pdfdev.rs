#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    pub type pdf_obj;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn modf(_: f64, _: *mut f64) -> f64;
    #[no_mangle]
    fn floor(_: f64) -> f64;
    #[no_mangle]
    fn round(_: f64) -> f64;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: libc::c_uint,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn abs(_: i32) -> i32;
    #[no_mangle]
    fn labs(_: i64) -> i64;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    /* Color stack
     */
    #[no_mangle]
    fn pdf_color_clear_stack();
    #[no_mangle]
    fn pdf_color_get_current(sc: *mut *mut pdf_color, fc: *mut *mut pdf_color);
    #[no_mangle]
    fn pdfobj_escape_str(
        buffer: *mut i8,
        size: size_t,
        s: *const u8,
        len: size_t,
    ) -> size_t;
    #[no_mangle]
    fn cff_charsets_lookup_cid(charset: *mut cff_charsets, gid: card16) -> card16;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    static mut work_buffer: [i8; 0];
    #[no_mangle]
    fn CMap_decode(
        cmap: *mut CMap,
        inbuf: *mut *const u8,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut u8,
        outbytesleft: *mut size_t,
    ) -> size_t;
    #[no_mangle]
    fn CMap_cache_get(id: i32) -> *mut CMap;
    #[no_mangle]
    fn pdf_lookup_fontmap_record(kp: *const i8) -> *mut fontmap_rec;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    /* allow other modules (pdfdev) to ask whether we're collecting box areas */
    #[no_mangle]
    fn dvi_is_tracking_boxes() -> bool;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn pdf_doc_add_page_content(buffer: *const i8, length: libc::c_uint);
    #[no_mangle]
    fn pdf_doc_add_page_resource(
        category: *const i8,
        resource_name: *const i8,
        resources: *mut pdf_obj,
    );
    #[no_mangle]
    fn pdf_doc_expand_box(rect: *const pdf_rect);
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn pdf_dev_init_gstates();
    #[no_mangle]
    fn pdf_dev_clear_gstates();
    #[no_mangle]
    fn pdf_dev_rectclip(
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    ) -> i32;
    #[no_mangle]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> i32;
    #[no_mangle]
    fn pdf_dev_transform(p: *mut pdf_coord, M: *const pdf_tmatrix);
    #[no_mangle]
    fn pdf_dev_gsave() -> i32;
    #[no_mangle]
    fn pdf_dev_grestore() -> i32;
    /* The depth here is the depth of q/Q nesting.
     * We must remember current depth of nesting when starting a page or xform,
     * and must recover until that depth at the end of page/xform.
     */
    #[no_mangle]
    fn pdf_dev_current_depth() -> i32;
    #[no_mangle]
    fn pdf_dev_grestore_to(depth: i32);
    #[no_mangle]
    fn pdf_dev_set_color(color: *const pdf_color, mask: i8, force: i32);
    /* font_name is used when mrec is NULL.
     * font_scale (point size) used by PK font.
     * It might be necessary if dvipdfmx supports font format with
     * various optical sizes supported in the future.
     */
    #[no_mangle]
    fn pdf_font_findresource(
        font_name: *const i8,
        font_scale: f64,
        mrec: *mut fontmap_rec,
    ) -> i32;
    #[no_mangle]
    fn pdf_get_font_subtype(font_id: i32) -> i32;
    #[no_mangle]
    fn pdf_get_font_reference(font_id: i32) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_get_font_usedchars(font_id: i32) -> *mut i8;
    #[no_mangle]
    fn pdf_get_font_encoding(font_id: i32) -> i32;
    #[no_mangle]
    fn pdf_get_font_wmode(font_id: i32) -> i32;
    #[no_mangle]
    fn pdf_ximage_get_resname(xobj_id: i32) -> *mut i8;
    #[no_mangle]
    fn pdf_ximage_get_reference(xobj_id: i32) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_ximage_scale_image(
        id: i32,
        M: *mut pdf_tmatrix,
        r: *mut pdf_rect,
        p: *mut transform_info,
    ) -> i32;
}
pub type size_t = u64;
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_color {
    pub num_components: i32,
    pub spot_color_name: *mut i8,
    pub values: [f64; 4],
}
pub type spt_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_tmatrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: f64,
    pub lly: f64,
    pub urx: f64,
    pub ury: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_coord {
    pub x: f64,
    pub y: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_info {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_font {
    pub short_name: [i8; 7],
    pub used_on_this_page: i32,
    pub tex_name: *mut i8,
    pub sptsize: spt_t,
    pub font_id: i32,
    pub enc_id: i32,
    pub real_font_index: i32,
    pub resource: *mut pdf_obj,
    pub used_chars: *mut i8,
    pub format: i32,
    pub wmode: i32,
    pub extend: f64,
    pub slant: f64,
    pub bold: f64,
    pub mapc: i32,
    pub ucs_group: i32,
    pub ucs_plane: i32,
    pub is_unicode: i32,
    pub cff_charsets: *mut cff_charsets,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_charsets {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub glyphs: *mut s_SID,
    pub range1: *mut cff_range1,
    pub range2: *mut cff_range2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range2 {
    pub first: s_SID,
    pub n_left: card16,
}
pub type card16 = u16;
pub type s_SID = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range1 {
    pub first: s_SID,
    pub n_left: card8,
}
pub type card8 = u8;
/*
 * Unit conversion, formatting and others.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub dvi2pts: f64,
    pub min_bp_val: i32,
    pub precision: i32,
    /* Number of decimal digits (in fractional part) kept. */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub font_id: i32,
    pub offset: spt_t,
    pub ref_x: spt_t,
    pub ref_y: spt_t,
    pub raise: spt_t,
    pub leading: spt_t,
    pub matrix: C2RustUnnamed_2,
    pub bold_param: f64,
    pub dir_mode: i32,
    pub force_reset: i32,
    pub is_mb: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub slant: f64,
    pub extend: f64,
    pub rotate: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub autorotate: i32,
    pub colormode: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut i8,
    pub type_0: i32,
    pub wmode: i32,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_5,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: i32,
    pub profile: C2RustUnnamed_4,
    pub reverseMap: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut u8,
    pub prev: *mut mapData,
    pub pos: i32,
    /* Position of next free data segment */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: i32,
    pub len: size_t,
    pub code: *mut u8,
    pub next: *mut mapDef,
    /* Next Subtbl for LOOKUP_CONTINUE */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub num: libc::c_uint,
    pub max: libc::c_uint,
    pub ranges: *mut rangeDef,
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/* Mapping types, MAP_IS_NAME is not supported. */
/* Lookup flags */
/* DEBUG */
/* Codespacerange */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut u8,
    pub codeHi: *mut u8,
    /* Upper bounds of valid input code */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut i8,
    pub ordering: *mut i8,
    pub supplement: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_opt {
    pub slant: f64,
    pub extend: f64,
    pub bold: f64,
    pub mapc: i32,
    pub flags: i32,
    pub otl_tags: *mut i8,
    pub tounicode: *mut i8,
    pub cff_charsets: *mut libc::c_void,
    pub design_size: f64,
    pub charcoll: *mut i8,
    pub index: i32,
    pub style: i32,
    pub stemv: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_rec {
    pub map_name: *mut i8,
    pub font_name: *mut i8,
    pub enc_name: *mut i8,
    pub charmap: C2RustUnnamed_6,
    pub opt: fontmap_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub sfd_name: *mut i8,
    pub subfont_id: *mut i8,
}
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const i8, mut s2: *const i8) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
static mut verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_set_verbose(mut level: i32) {
    verbose = level;
}
/* Not working yet... */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_scale() -> f64 {
    return 1.0f64;
}
static mut dev_unit: C2RustUnnamed_0 = {
    let mut init = C2RustUnnamed_0 {
        dvi2pts: 0.0f64,
        min_bp_val: 658i32,
        precision: 2i32,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn dev_unit_dviunit() -> f64 {
    return 1.0f64 / dev_unit.dvi2pts;
}
static mut ten_pow: [u32; 10] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
];
static mut ten_pow_inv: [f64; 10] = [
    1.0f64,
    0.1f64,
    0.01f64,
    0.001f64,
    0.0001f64,
    0.00001f64,
    0.000001f64,
    0.0000001f64,
    0.00000001f64,
    0.000000001f64,
];
unsafe extern "C" fn p_itoa(mut value: i32, mut buf: *mut i8) -> libc::c_uint {
    let mut sign: libc::c_uint = 0;
    let mut ndigits: libc::c_uint = 0;
    let mut p: *mut i8 = buf;
    if value < 0i32 {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = '-' as i32 as i8;
        value = -value;
        sign = 1i32 as libc::c_uint
    } else {
        sign = 0i32 as libc::c_uint
    }
    ndigits = 0i32 as libc::c_uint;
    loop
    /* Generate at least one digit in reverse order */
    {
        let fresh1 = ndigits;
        ndigits = ndigits.wrapping_add(1);
        *p.offset(fresh1 as isize) = (value % 10i32 + '0' as i32) as i8;
        value /= 10i32;
        if !(value != 0i32) {
            break;
        }
    }
    /* Reverse the digits */
    let mut i: libc::c_uint = 0;
    i = 0i32 as libc::c_uint;
    while i < ndigits.wrapping_div(2i32 as libc::c_uint) {
        let mut tmp: i8 = *p.offset(i as isize);
        *p.offset(i as isize) =
            *p.offset(ndigits.wrapping_sub(i).wrapping_sub(1i32 as libc::c_uint) as isize);
        *p.offset(ndigits.wrapping_sub(i).wrapping_sub(1i32 as libc::c_uint) as isize) = tmp;
        i = i.wrapping_add(1)
    }
    *p.offset(ndigits as isize) = '\u{0}' as i32 as i8;
    return if sign != 0 {
        ndigits.wrapping_add(1i32 as libc::c_uint)
    } else {
        ndigits
    };
}
/* NOTE: Acrobat 5 and prior uses 16.16 fixed point representation for
 * real numbers.
 */
unsafe extern "C" fn p_dtoa(
    mut value: f64,
    mut prec: i32,
    mut buf: *mut i8,
) -> i32 {
    let p: [i32; 10] = [
        1i32,
        10i32,
        100i32,
        1000i32,
        10000i32,
        100000i32,
        1000000i32,
        10000000i32,
        100000000i32,
        1000000000i32,
    ];
    let mut i: f64 = 0.;
    let mut f: f64 = 0.;
    let mut g: i32 = 0;
    let mut c: *mut i8 = buf;
    let mut n: i32 = 0;
    if value < 0i32 as f64 {
        value = -value;
        let fresh2 = c;
        c = c.offset(1);
        *fresh2 = '-' as i32 as i8;
        n = 1i32
    } else {
        n = 0i32
    }
    f = modf(value, &mut i);
    g = (f * p[prec as usize] as f64 + 0.5f64) as i32;
    if g == p[prec as usize] {
        g = 0i32;
        i += 1i32 as f64
    }
    if i != 0. {
        let mut m: i32 = sprintf(c, b"%.0f\x00" as *const u8 as *const i8, i);
        c = c.offset(m as isize);
        n += m
    } else if g == 0i32 {
        c = buf;
        *c = '0' as i32 as i8;
        n = 1i32
    }
    if g != 0 {
        let mut j: i32 = prec;
        let fresh3 = c;
        c = c.offset(1);
        *fresh3 = '.' as i32 as i8;
        loop {
            let fresh4 = j;
            j = j - 1;
            if !(fresh4 != 0) {
                break;
            }
            *c.offset(j as isize) = (g % 10i32 + '0' as i32) as i8;
            g /= 10i32
        }
        c = c.offset((prec - 1i32) as isize);
        n += 1i32 + prec;
        while *c as i32 == '0' as i32 {
            c = c.offset(-1);
            n -= 1
        }
    }
    c = c.offset(1);
    *c = 0i32 as i8;
    return n;
}
unsafe extern "C" fn dev_sprint_bp(
    mut buf: *mut i8,
    mut value: spt_t,
    mut error: *mut spt_t,
) -> i32 {
    let mut value_in_bp: f64 = 0.;
    let mut error_in_bp: f64 = 0.;
    let mut prec: i32 = dev_unit.precision;
    value_in_bp = value as f64 * dev_unit.dvi2pts;
    if !error.is_null() {
        error_in_bp = value_in_bp
            - floor(value_in_bp / ten_pow_inv[prec as usize] + 0.5f64) * ten_pow_inv[prec as usize];
        *error = round(error_in_bp / dev_unit.dvi2pts) as spt_t
    }
    return p_dtoa(value_in_bp, prec, buf);
}
/* They are affected by precision (set at device initialization). */
#[no_mangle]
pub unsafe extern "C" fn pdf_sprint_matrix(
    mut buf: *mut i8,
    mut M: *const pdf_tmatrix,
) -> i32 {
    let mut len: i32 = 0; /* xxx_sprint_xxx NULL terminates strings. */
    let mut prec2: i32 = if dev_unit.precision + 2i32 < 8i32 {
        dev_unit.precision + 2i32
    } else {
        8i32
    }; /* xxx_sprint_xxx NULL terminates strings. */
    let mut prec0: i32 = if dev_unit.precision > 2i32 {
        dev_unit.precision
    } else {
        2i32
    }; /* xxx_sprint_xxx NULL terminates strings. */
    len = p_dtoa((*M).a, prec2, buf); /* xxx_sprint_xxx NULL terminates strings. */
    let fresh5 = len; /* xxx_sprint_xxx NULL terminates strings. */
    len = len + 1;
    *buf.offset(fresh5 as isize) = ' ' as i32 as i8;
    len += p_dtoa((*M).b, prec2, buf.offset(len as isize));
    let fresh6 = len;
    len = len + 1;
    *buf.offset(fresh6 as isize) = ' ' as i32 as i8;
    len += p_dtoa((*M).c, prec2, buf.offset(len as isize));
    let fresh7 = len;
    len = len + 1;
    *buf.offset(fresh7 as isize) = ' ' as i32 as i8;
    len += p_dtoa((*M).d, prec2, buf.offset(len as isize));
    let fresh8 = len;
    len = len + 1;
    *buf.offset(fresh8 as isize) = ' ' as i32 as i8;
    len += p_dtoa((*M).e, prec0, buf.offset(len as isize));
    let fresh9 = len;
    len = len + 1;
    *buf.offset(fresh9 as isize) = ' ' as i32 as i8;
    len += p_dtoa((*M).f, prec0, buf.offset(len as isize));
    *buf.offset(len as isize) = '\u{0}' as i32 as i8;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_sprint_rect(
    mut buf: *mut i8,
    mut rect: *const pdf_rect,
) -> i32 {
    let mut len: i32 = 0;
    len = p_dtoa((*rect).llx, dev_unit.precision, buf);
    let fresh10 = len;
    len = len + 1;
    *buf.offset(fresh10 as isize) = ' ' as i32 as i8;
    len += p_dtoa((*rect).lly, dev_unit.precision, buf.offset(len as isize));
    let fresh11 = len;
    len = len + 1;
    *buf.offset(fresh11 as isize) = ' ' as i32 as i8;
    len += p_dtoa((*rect).urx, dev_unit.precision, buf.offset(len as isize));
    let fresh12 = len;
    len = len + 1;
    *buf.offset(fresh12 as isize) = ' ' as i32 as i8;
    len += p_dtoa((*rect).ury, dev_unit.precision, buf.offset(len as isize));
    *buf.offset(len as isize) = '\u{0}' as i32 as i8;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_sprint_coord(
    mut buf: *mut i8,
    mut p: *const pdf_coord,
) -> i32 {
    let mut len: i32 = 0;
    len = p_dtoa((*p).x, dev_unit.precision, buf);
    let fresh13 = len;
    len = len + 1;
    *buf.offset(fresh13 as isize) = ' ' as i32 as i8;
    len += p_dtoa((*p).y, dev_unit.precision, buf.offset(len as isize));
    *buf.offset(len as isize) = '\u{0}' as i32 as i8;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_sprint_length(
    mut buf: *mut i8,
    mut value: f64,
) -> i32 {
    let mut len: i32 = 0;
    len = p_dtoa(value, dev_unit.precision, buf);
    *buf.offset(len as isize) = '\u{0}' as i32 as i8;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_sprint_number(
    mut buf: *mut i8,
    mut value: f64,
) -> i32 {
    let mut len: i32 = 0;
    len = p_dtoa(value, 8i32, buf);
    *buf.offset(len as isize) = '\u{0}' as i32 as i8;
    return len;
}
static mut dev_param: C2RustUnnamed_3 = {
    let mut init = C2RustUnnamed_3 {
        autorotate: 1i32,
        colormode: 1i32,
    };
    init
};
static mut motion_state: i32 = 1i32;
static mut format_buffer: [i8; 4096] = [0; 4096];
static mut text_state: C2RustUnnamed_1 = {
    let mut init = C2RustUnnamed_1 {
        font_id: -1i32,
        offset: 0i32,
        ref_x: 0i32,
        ref_y: 0i32,
        raise: 0i32,
        leading: 0i32,
        matrix: {
            let mut init = C2RustUnnamed_2 {
                slant: 0.0f64,
                extend: 1.0f64,
                rotate: 0i32,
            };
            init
        },
        bold_param: 0.0f64,
        dir_mode: 0i32,
        force_reset: 0i32,
        is_mb: 0i32,
    };
    init
};
static mut dev_fonts: *mut dev_font = 0 as *const dev_font as *mut dev_font;
static mut num_dev_fonts: i32 = 0i32;
static mut max_dev_fonts: i32 = 0i32;
static mut num_phys_fonts: i32 = 0i32;
unsafe extern "C" fn dev_set_text_matrix(
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut slant: f64,
    mut extend: f64,
    mut rotate: i32,
) {
    let mut tm: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut len: i32 = 0i32;
    /* slant is negated for vertical font so that right-side
     * is always lower. */
    match rotate {
        4 => {
            /* Vertical font */
            tm.a = slant;
            tm.b = 1.0f64;
            tm.c = -extend;
            tm.d = 0.0f64
        }
        1 => {
            /* Horizontal font */
            tm.a = 0.0f64;
            tm.b = -extend;
            tm.c = 1.0f64;
            tm.d = -slant
        }
        0 => {
            /* Horizontal font */
            tm.a = extend;
            tm.b = 0.0f64;
            tm.c = slant;
            tm.d = 1.0f64
        }
        5 => {
            /* Vertical font */
            tm.a = 1.0f64;
            tm.b = -slant;
            tm.c = 0.0f64;
            tm.d = extend
        }
        3 => {
            /* Horizontal font */
            tm.a = 0.0f64;
            tm.b = extend;
            tm.c = -1.0f64;
            tm.d = slant
        }
        7 => {
            /* Vertical font */
            tm.a = -1.0f64; /* op: Tm */
            tm.b = slant;
            tm.c = 0.0f64;
            tm.d = -extend
        }
        _ => {}
    }
    tm.e = xpos as f64 * dev_unit.dvi2pts;
    tm.f = ypos as f64 * dev_unit.dvi2pts;
    let fresh14 = len;
    len = len + 1;
    format_buffer[fresh14 as usize] = ' ' as i32 as i8;
    len += pdf_sprint_matrix(format_buffer.as_mut_ptr().offset(len as isize), &mut tm);
    let fresh15 = len;
    len = len + 1;
    format_buffer[fresh15 as usize] = ' ' as i32 as i8;
    let fresh16 = len;
    len = len + 1;
    format_buffer[fresh16 as usize] = 'T' as i32 as i8;
    let fresh17 = len;
    len = len + 1;
    format_buffer[fresh17 as usize] = 'm' as i32 as i8;
    pdf_doc_add_page_content(format_buffer.as_mut_ptr(), len as libc::c_uint);
    text_state.ref_x = xpos;
    text_state.ref_y = ypos;
    text_state.matrix.slant = slant;
    text_state.matrix.extend = extend;
    text_state.matrix.rotate = rotate;
}
/*
 * reset_text_state() outputs a BT and does any necessary coordinate
 * transformations to get ready to ship out text.
 */
unsafe extern "C" fn reset_text_state() {
    /*
     * We need to reset the line matrix to handle slanted fonts.
     */
    pdf_doc_add_page_content(
        b" BT\x00" as *const u8 as *const i8,
        3i32 as libc::c_uint,
    ); /* op: BT */
    /*
     * text_state.matrix is identity at top of page.
     * This sometimes write unnecessary "Tm"s when transition from
     * GRAPHICS_MODE to TEXT_MODE occurs.
     */
    if text_state.force_reset != 0
        || text_state.matrix.slant != 0.0f64
        || text_state.matrix.extend != 1.0f64
        || text_state.matrix.rotate != 0i32 && text_state.matrix.rotate != 5i32
    {
        dev_set_text_matrix(
            0i32,
            0i32,
            text_state.matrix.slant,
            text_state.matrix.extend,
            text_state.matrix.rotate,
        ); /* op: TJ */
    } /* op: TJ */
    text_state.ref_x = 0i32;
    text_state.ref_y = 0i32;
    text_state.offset = 0i32;
    text_state.force_reset = 0i32;
}
unsafe extern "C" fn text_mode() {
    match motion_state {
        3 => {
            pdf_doc_add_page_content(
                if text_state.is_mb != 0 {
                    b">]TJ\x00" as *const u8 as *const i8
                } else {
                    b")]TJ\x00" as *const u8 as *const i8
                },
                4i32 as libc::c_uint,
            );
        }
        1 => {
            reset_text_state();
        }
        2 | _ => {}
    }
    motion_state = 2i32;
    text_state.offset = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn graphics_mode() {
    let mut current_block_3: u64;
    match motion_state {
        3 => {
            pdf_doc_add_page_content(
                if text_state.is_mb != 0 {
                    b">]TJ\x00" as *const u8 as *const i8
                } else {
                    b")]TJ\x00" as *const u8 as *const i8
                },
                4i32 as libc::c_uint,
            );
            current_block_3 = 13064676843759196241;
        }
        2 => {
            current_block_3 = 13064676843759196241;
        }
        1 | _ => {
            current_block_3 = 11875828834189669668;
        }
    }
    match current_block_3 {
        13064676843759196241 =>
        /* continue */
        {
            pdf_doc_add_page_content(
                b" ET\x00" as *const u8 as *const i8,
                3i32 as libc::c_uint,
            ); /* op: ET */
            text_state.force_reset = 0i32;
            text_state.font_id = -1i32
        }
        _ => {}
    }
    motion_state = 1i32;
}
unsafe extern "C" fn start_string(
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut slant: f64,
    mut extend: f64,
    mut rotate: i32,
) {
    let mut delx: spt_t = 0;
    let mut dely: spt_t = 0;
    let mut error_delx: spt_t = 0i32;
    let mut error_dely: spt_t = 0i32;
    let mut desired_delx: spt_t = 0;
    let mut desired_dely: spt_t = 0;
    let mut len: i32 = 0i32;
    delx = xpos - text_state.ref_x;
    dely = ypos - text_state.ref_y;
    /*
     * Precompensating for line transformation matrix.
     *
     * Line transformation matrix L for horizontal font in horizontal
     * mode and it's inverse I is
     *
     *          | e  0|          | 1/e  0|
     *   L_hh = |     | , I_hh = |       |
     *          | s  1|          |-s/e  1|
     *
     * For vertical font in vertical mode,
     *
     *          | 1  -s|          | 1  s/e|
     *   L_vv = |      | , I_vv = |       |
     *          | 0   e|          | 0  1/e|
     *
     * For vertical font in horizontal mode,
     *
     *          | s   1|          | 0  1|
     *   L_vh = |      | = L_vv x |     |
     *          |-e   0|          |-1  0|
     *
     *          | 0  -1|
     *   I_vh = |      | x I_vv
     *          | 1   0|
     *
     * For horizontal font in vertical mode,
     *
     *          | 0  -e|          | 0  -1|
     *   L_hv = |      | = L_hh x |      |
     *          | 1  -s|          | 1   0|
     *
     *          | 0   1|
     *   I_hv = |      | x I_hh
     *          |-1   0|
     *
     */
    match rotate {
        4 => {
            /* Vertical font in horizontal mode: rot = +90
             *                           | 0  -1/e|
             * d_user =  d x I_vh = d x  |        |
             *                           | 1   s/e|
             */
            desired_delx = dely;
            desired_dely =
                (-(delx as f64 - dely as f64 * slant) / extend) as spt_t;
            /* error_del is in device space
             *
             *               | 0  1|
             *  e = e_user x |     | = (-e_user_y, e_user_x)
             *               |-1  0|
             *
             * We must care about rotation here but not extend/slant...
             * The extend and slant actually is font matrix.
             */
            let fresh18 = len;
            len = len + 1;
            format_buffer[fresh18 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_delx,
                &mut error_dely,
            );
            let fresh19 = len;
            len = len + 1;
            format_buffer[fresh19 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_dely,
                &mut error_delx,
            );
            error_delx = -error_delx
        }
        1 => {
            /* Horizontal font in vertical mode: rot = -90
             *
             *                         |-s/e  1|
             * d_user = d x I_hv = d x |       |
             *                         |-1/e  0|
             */
            desired_delx =
                (-(dely as f64 + delx as f64 * slant) / extend) as spt_t;
            desired_dely = delx;
            /*
             * e = (e_user_y, -e_user_x)
             */
            let fresh20 = len;
            len = len + 1;
            format_buffer[fresh20 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_delx,
                &mut error_dely,
            );
            let fresh21 = len;
            len = len + 1;
            format_buffer[fresh21 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_dely,
                &mut error_delx,
            );
            error_dely = -error_dely
        }
        0 => {
            /* Horizontal font in horizontal mode:
             *                         | 1/e    0|
             * d_user = d x I_hh = d x |         |
             *                         |-s/e    1|
             */
            desired_delx =
                ((delx as f64 - dely as f64 * slant) / extend) as spt_t;
            desired_dely = dely;
            let fresh22 = len;
            len = len + 1;
            format_buffer[fresh22 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_delx,
                &mut error_delx,
            );
            let fresh23 = len;
            len = len + 1;
            format_buffer[fresh23 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_dely,
                &mut error_dely,
            )
        }
        5 => {
            /* Vertical font in vertical mode:
             *                         | 1  s/e|
             * d_user = d x I_vv = d x |       |
             *                         | 0  1/e|
             */
            desired_delx = delx;
            desired_dely =
                ((dely as f64 + delx as f64 * slant) / extend) as spt_t;
            let fresh24 = len;
            len = len + 1;
            format_buffer[fresh24 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_delx,
                &mut error_delx,
            );
            let fresh25 = len;
            len = len + 1;
            format_buffer[fresh25 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_dely,
                &mut error_dely,
            )
        }
        3 => {
            /* Horizontal font in down-to-up mode: rot = +90
             *
             *                          | s/e  -1|
             * d_user = d x -I_hv = d x |        |
             *                          | 1/e   0|
             */
            desired_delx =
                -((-(dely as f64 + delx as f64 * slant) / extend) as spt_t);
            desired_dely = -delx;
            let fresh26 = len;
            len = len + 1;
            format_buffer[fresh26 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_delx,
                &mut error_dely,
            );
            let fresh27 = len;
            len = len + 1;
            format_buffer[fresh27 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_dely,
                &mut error_delx,
            );
            error_delx = -error_delx;
            error_dely = -error_dely
        }
        7 => {
            /* Vertical font in down-to-up mode: rot = 180
             *                          |-1 -s/e|
             * d_user = d x -I_vv = d x |       |
             *                          | 0 -1/e|
             */
            desired_delx = -delx; /* op: */
            desired_dely =
                -(((dely as f64 + delx as f64 * slant) / extend) as spt_t);
            let fresh28 = len;
            len = len + 1;
            format_buffer[fresh28 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_delx,
                &mut error_delx,
            );
            let fresh29 = len;
            len = len + 1;
            format_buffer[fresh29 as usize] = ' ' as i32 as i8;
            len += dev_sprint_bp(
                format_buffer.as_mut_ptr().offset(len as isize),
                desired_dely,
                &mut error_dely,
            );
            error_delx = -error_delx;
            error_dely = -error_dely
        }
        _ => {}
    }
    pdf_doc_add_page_content(format_buffer.as_mut_ptr(), len as libc::c_uint);
    /*
     * dvipdfm wrongly using "TD" in place of "Td".
     * The TD operator set leading, but we are not using T* etc.
     */
    pdf_doc_add_page_content(
        if text_state.is_mb != 0 {
            b" Td[<\x00" as *const u8 as *const i8
        } else {
            b" Td[(\x00" as *const u8 as *const i8
        },
        5i32 as libc::c_uint,
    ); /* op: Td */
    /* Error correction */
    text_state.ref_x = xpos - error_delx;
    text_state.ref_y = ypos - error_dely;
    text_state.offset = 0i32;
}
unsafe extern "C" fn string_mode(
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut slant: f64,
    mut extend: f64,
    mut rotate: i32,
) {
    let mut current_block_7: u64;
    match motion_state {
        1 => {
            reset_text_state();
            current_block_7 = 1909977495246269370;
        }
        2 => {
            current_block_7 = 1909977495246269370;
        }
        3 | _ => {
            current_block_7 = 3640593987805443782;
        }
    }
    match current_block_7 {
        1909977495246269370 =>
        /* continue */
        {
            if text_state.force_reset != 0 {
                dev_set_text_matrix(xpos, ypos, slant, extend, rotate); /* op: */
                pdf_doc_add_page_content(
                    if text_state.is_mb != 0 {
                        b"[<\x00" as *const u8 as *const i8
                    } else {
                        b"[(\x00" as *const u8 as *const i8
                    },
                    2i32 as libc::c_uint,
                );
                text_state.force_reset = 0i32
            } else {
                start_string(xpos, ypos, slant, extend, rotate);
            }
        }
        _ => {}
    }
    motion_state = 3i32;
}
/*
 * The purpose of the following routine is to force a Tf only
 * when it's actually necessary.  This became a problem when the
 * VF code was added.  The VF spec says to instantiate the
 * first font contained in the VF file before drawing a virtual
 * character.  However, that font may not be used for
 * many characters (e.g. small caps fonts).  For this reason,
 * dev_select_font() should not force a "physical" font selection.
 * This routine prevents a PDF Tf font selection until there's
 * really a character in that font.
 */
unsafe extern "C" fn dev_set_font(mut font_id: i32) -> i32 {
    let mut font: *mut dev_font = 0 as *mut dev_font;
    let mut real_font: *mut dev_font = 0 as *mut dev_font;
    let mut text_rotate: i32 = 0;
    let mut font_scale: f64 = 0.;
    let mut len: i32 = 0;
    let mut vert_dir: i32 = 0;
    let mut vert_font: i32 = 0;
    /* text_mode() must come before text_state.is_mb is changed. */
    text_mode(); /* Caller should check font_id. */
    font = &mut *dev_fonts.offset(font_id as isize) as *mut dev_font; /* space not necessary. */
    if !font.is_null() {
    } else {
        __assert_fail(
            b"font\x00" as *const u8 as *const i8,
            b"dpx-pdfdev.c\x00" as *const u8 as *const i8,
            847i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 22], &[i8; 22]>(
                b"int dev_set_font(int)\x00",
            ))
            .as_ptr(),
        ); /* op: Tf */
    } /* _FIXME_ */
    if (*font).real_font_index >= 0i32 {
        real_font = &mut *dev_fonts.offset((*font).real_font_index as isize) as *mut dev_font
    } else {
        real_font = font
    }
    text_state.is_mb = if (*font).format == 3i32 { 1i32 } else { 0i32 };
    vert_font = if (*font).wmode != 0 { 1i32 } else { 0i32 };
    if dev_param.autorotate != 0 {
        vert_dir = text_state.dir_mode
    } else {
        vert_dir = vert_font
    }
    text_rotate = vert_font << 2i32 | vert_dir;
    if (*font).slant != text_state.matrix.slant
        || (*font).extend != text_state.matrix.extend
        || (if abs(text_rotate - text_state.matrix.rotate) % 5i32 == 0i32 {
            0i32
        } else {
            1i32
        }) != 0
    {
        text_state.force_reset = 1i32
    }
    text_state.matrix.slant = (*font).slant;
    text_state.matrix.extend = (*font).extend;
    text_state.matrix.rotate = text_rotate;
    if (*real_font).resource.is_null() {
        (*real_font).resource = pdf_get_font_reference((*real_font).font_id);
        (*real_font).used_chars = pdf_get_font_usedchars((*real_font).font_id)
    }
    if (*real_font).used_on_this_page == 0 {
        pdf_doc_add_page_resource(
            b"Font\x00" as *const u8 as *const i8,
            (*real_font).short_name.as_mut_ptr(),
            pdf_link_obj((*real_font).resource),
        );
        (*real_font).used_on_this_page = 1i32
    }
    font_scale = (*font).sptsize as f64 * dev_unit.dvi2pts;
    len = sprintf(
        format_buffer.as_mut_ptr(),
        b" /%s\x00" as *const u8 as *const i8,
        (*real_font).short_name.as_mut_ptr(),
    );
    let fresh30 = len;
    len = len + 1;
    format_buffer[fresh30 as usize] = ' ' as i32 as i8;
    len += p_dtoa(
        font_scale,
        if dev_unit.precision + 1i32 < 8i32 {
            dev_unit.precision + 1i32
        } else {
            8i32
        },
        format_buffer.as_mut_ptr().offset(len as isize),
    );
    let fresh31 = len;
    len = len + 1;
    format_buffer[fresh31 as usize] = ' ' as i32 as i8;
    let fresh32 = len;
    len = len + 1;
    format_buffer[fresh32 as usize] = 'T' as i32 as i8;
    let fresh33 = len;
    len = len + 1;
    format_buffer[fresh33 as usize] = 'f' as i32 as i8;
    pdf_doc_add_page_content(format_buffer.as_mut_ptr(), len as libc::c_uint);
    if (*font).bold > 0.0f64 || (*font).bold != text_state.bold_param {
        if (*font).bold <= 0.0f64 {
            len = sprintf(
                format_buffer.as_mut_ptr(),
                b" 0 Tr\x00" as *const u8 as *const i8,
            )
        } else {
            len = sprintf(
                format_buffer.as_mut_ptr(),
                b" 2 Tr %.6f w\x00" as *const u8 as *const i8,
                (*font).bold,
            )
        }
        pdf_doc_add_page_content(format_buffer.as_mut_ptr(), len as libc::c_uint);
        /* op: Tr w */
    }
    text_state.bold_param = (*font).bold;
    text_state.font_id = font_id;
    return 0i32;
}
/* Access text state parameters.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_get_font_wmode(mut font_id: i32) -> i32 {
    let mut font: *mut dev_font = 0 as *mut dev_font;
    font = &mut *dev_fonts.offset(font_id as isize) as *mut dev_font;
    if !font.is_null() {
        return (*font).wmode;
    }
    return 0i32;
}
static mut sbuf0: [u8; 4096] = [0; 4096];
static mut sbuf1: [u8; 4096] = [0; 4096];
unsafe extern "C" fn handle_multibyte_string(
    mut font: *mut dev_font,
    mut str_ptr: *mut *const u8,
    mut str_len: *mut size_t,
    mut ctype: i32,
) -> i32 {
    let mut p: *const u8 = 0 as *const u8;
    let mut i: size_t = 0;
    let mut length: size_t = 0;
    p = *str_ptr;
    length = *str_len;
    if ctype == -1i32 && !(*font).cff_charsets.is_null() {
        /* freetype glyph indexes */
        /* Convert freetype glyph indexes to CID. */
        let mut inbuf: *const u8 = p;
        let mut outbuf: *mut u8 = sbuf0.as_mut_ptr();
        i = 0i32 as size_t;
        while i < length {
            let mut gid: libc::c_uint = 0;
            let fresh34 = inbuf;
            inbuf = inbuf.offset(1);
            gid = ((*fresh34 as i32) << 8i32) as libc::c_uint;
            let fresh35 = inbuf;
            inbuf = inbuf.offset(1);
            gid = gid.wrapping_add(*fresh35 as libc::c_uint);
            gid = cff_charsets_lookup_cid((*font).cff_charsets, gid as card16) as libc::c_uint;
            let fresh36 = outbuf;
            outbuf = outbuf.offset(1);
            *fresh36 = (gid >> 8i32) as u8;
            let fresh37 = outbuf;
            outbuf = outbuf.offset(1);
            *fresh37 = (gid & 0xffi32 as libc::c_uint) as u8;
            i = (i as u64).wrapping_add(2i32 as u64) as size_t as size_t
        }
        p = sbuf0.as_mut_ptr();
        length = outbuf.wrapping_offset_from(sbuf0.as_mut_ptr()) as i64 as size_t
    } else if (*font).is_unicode != 0 {
        /* _FIXME_ */
        /* UCS-4 */
        if ctype == 1i32 {
            if length.wrapping_mul(4i32 as u64) >= 4096i32 as u64 {
                dpx_warning(b"Too long string...\x00" as *const u8 as *const i8);
                return -1i32;
            }
            i = 0i32 as size_t;
            while i < length {
                sbuf1[i.wrapping_mul(4i32 as u64) as usize] =
                    (*font).ucs_group as u8;
                sbuf1[i
                    .wrapping_mul(4i32 as u64)
                    .wrapping_add(1i32 as u64) as usize] =
                    (*font).ucs_plane as u8;
                sbuf1[i
                    .wrapping_mul(4i32 as u64)
                    .wrapping_add(2i32 as u64) as usize] =
                    '\u{0}' as i32 as u8;
                sbuf1[i
                    .wrapping_mul(4i32 as u64)
                    .wrapping_add(3i32 as u64) as usize] = *p.offset(i as isize);
                i = i.wrapping_add(1)
            }
            length =
                (length as u64).wrapping_mul(4i32 as u64) as size_t as size_t
        } else if ctype == 2i32 {
            let mut len: size_t = 0i32 as size_t;
            if length.wrapping_mul(2i32 as u64) >= 4096i32 as u64 {
                dpx_warning(b"Too long string...\x00" as *const u8 as *const i8);
                return -1i32;
            }
            i = 0i32 as size_t;
            while i < length {
                sbuf1[len as usize] = (*font).ucs_group as u8;
                if *p.offset(i as isize) as i32 & 0xf8i32 == 0xd8i32 {
                    let mut c: i32 = 0;
                    /* Check for valid surrogate pair.  */
                    if *p.offset(i as isize) as i32 & 0xfci32 != 0xd8i32
                        || i.wrapping_add(2i32 as u64) >= length
                        || *p.offset(i.wrapping_add(2i32 as u64) as isize) as i32
                            & 0xfci32
                            != 0xdci32
                    {
                        dpx_warning(
                            b"Invalid surrogate p[%zu]=%02X...\x00" as *const u8
                                as *const i8,
                            i,
                            *p.offset(i as isize) as i32,
                        );
                        return -1i32;
                    }
                    c = ((*p.offset(i as isize) as i32 & 0x3i32) << 10i32
                        | (*p.offset(i.wrapping_add(1i32 as u64) as isize)
                            as i32)
                            << 2i32
                        | *p.offset(i.wrapping_add(2i32 as u64) as isize) as i32
                            & 0x3i32)
                        + 0x100i32;
                    sbuf1[len.wrapping_add(1i32 as u64) as usize] =
                        (c >> 8i32 & 0xffi32) as u8;
                    sbuf1[len.wrapping_add(2i32 as u64) as usize] =
                        (c & 0xffi32) as u8;
                    i = (i as u64).wrapping_add(2i32 as u64) as size_t as size_t
                } else {
                    sbuf1[len.wrapping_add(1i32 as u64) as usize] =
                        (*font).ucs_plane as u8;
                    sbuf1[len.wrapping_add(2i32 as u64) as usize] = *p.offset(i as isize)
                }
                sbuf1[len.wrapping_add(3i32 as u64) as usize] =
                    *p.offset(i.wrapping_add(1i32 as u64) as isize);
                i = (i as u64).wrapping_add(2i32 as u64) as size_t as size_t;
                len = (len as u64).wrapping_add(4i32 as u64) as size_t as size_t
            }
            length = len
        }
        p = sbuf1.as_mut_ptr()
    } else if ctype == 1i32 && (*font).mapc >= 0i32 {
        /* Omega workaround...
         * Translate single-byte chars to double byte code space.
         */
        if length.wrapping_mul(2i32 as u64) >= 4096i32 as u64 {
            dpx_warning(b"Too long string...\x00" as *const u8 as *const i8);
            return -1i32;
        }
        i = 0i32 as size_t;
        while i < length {
            sbuf1[i.wrapping_mul(2i32 as u64) as usize] =
                ((*font).mapc & 0xffi32) as u8;
            sbuf1[i
                .wrapping_mul(2i32 as u64)
                .wrapping_add(1i32 as u64) as usize] = *p.offset(i as isize);
            i = i.wrapping_add(1)
        }
        length = (length as u64).wrapping_mul(2i32 as u64) as size_t as size_t;
        p = sbuf1.as_mut_ptr()
    }
    /*
     * Font is double-byte font. Output is assumed to be 16-bit fixed length
     * encoding.
     * TODO: A character decomposed to multiple characters.
     */
    if ctype != -1i32 && (*font).enc_id >= 0i32 {
        let mut inbuf_0: *const u8 = 0 as *const u8;
        let mut outbuf_0: *mut u8 = 0 as *mut u8;
        let mut inbytesleft: size_t = 0;
        let mut outbytesleft: size_t = 0;
        let mut cmap: *mut CMap = 0 as *mut CMap;
        cmap = CMap_cache_get((*font).enc_id);
        inbuf_0 = p;
        outbuf_0 = sbuf0.as_mut_ptr();
        inbytesleft = length;
        outbytesleft = 4096i32 as size_t;
        CMap_decode(
            cmap,
            &mut inbuf_0,
            &mut inbytesleft,
            &mut outbuf_0,
            &mut outbytesleft,
        );
        if inbytesleft != 0i32 as u64 {
            dpx_warning(
                b"CMap conversion failed. (%zu bytes remains)\x00" as *const u8
                    as *const i8,
                inbytesleft,
            );
            return -1i32;
        }
        length = (4096i32 as u64).wrapping_sub(outbytesleft);
        p = sbuf0.as_mut_ptr()
    }
    *str_ptr = p;
    *str_len = length;
    return 0i32;
}
static mut dev_coords: *mut pdf_coord = 0 as *const pdf_coord as *mut pdf_coord;
static mut num_dev_coords: i32 = 0i32;
static mut max_dev_coords: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_get_coord(
    mut xpos: *mut f64,
    mut ypos: *mut f64,
) {
    if num_dev_coords > 0i32 {
        *xpos = (*dev_coords.offset((num_dev_coords - 1i32) as isize)).x;
        *ypos = (*dev_coords.offset((num_dev_coords - 1i32) as isize)).y
    } else {
        *ypos = 0.0f64;
        *xpos = *ypos
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_push_coord(mut xpos: f64, mut ypos: f64) {
    if num_dev_coords >= max_dev_coords {
        max_dev_coords += 4i32;
        dev_coords = renew(
            dev_coords as *mut libc::c_void,
            (max_dev_coords as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<pdf_coord>() as u64)
                as u32,
        ) as *mut pdf_coord
    }
    (*dev_coords.offset(num_dev_coords as isize)).x = xpos;
    (*dev_coords.offset(num_dev_coords as isize)).y = ypos;
    num_dev_coords += 1;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_pop_coord() {
    if num_dev_coords > 0i32 {
        num_dev_coords -= 1
    };
}
/*
 * ctype:
 *  -1 input string contains 2-byte Freetype glyph index values
 *     (XeTeX only)
 *  0  byte-width of char can be variable and input string
 *     is properly encoded.
 *  n  Single character cosumes n bytes in input string.
 *
 * _FIXME_
 * -->
 * selectfont(font_name, point_size) and show_string(pos, string)
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_set_string(
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut instr_ptr: *const libc::c_void,
    mut instr_len: size_t,
    mut width: spt_t,
    mut font_id: i32,
    mut ctype: i32,
) {
    let mut font: *mut dev_font = 0 as *mut dev_font; /* Pointer to the reencoded string. */
    let mut real_font: *mut dev_font = 0 as *mut dev_font;
    let mut str_ptr: *const u8 = 0 as *const u8;
    let mut length: size_t = 0;
    let mut i: size_t = 0;
    let mut len: size_t = 0i32 as size_t;
    let mut kern: spt_t = 0;
    let mut delh: spt_t = 0;
    let mut delv: spt_t = 0;
    let mut text_xorigin: spt_t = 0;
    let mut text_yorigin: spt_t = 0;
    if font_id < 0i32 || font_id >= num_dev_fonts {
        _tt_abort(
            b"Invalid font: %d (%d)\x00" as *const u8 as *const i8,
            font_id,
            num_dev_fonts,
        );
    }
    if font_id != text_state.font_id {
        dev_set_font(font_id);
    }
    font = if text_state.font_id < 0i32 {
        0 as *mut dev_font
    } else {
        &mut *dev_fonts.offset(text_state.font_id as isize) as *mut dev_font
    };
    if font.is_null() {
        _tt_abort(b"Currentfont not set.\x00" as *const u8 as *const i8);
    }
    if (*font).real_font_index >= 0i32 {
        real_font = &mut *dev_fonts.offset((*font).real_font_index as isize) as *mut dev_font
    } else {
        real_font = font
    }
    text_xorigin = text_state.ref_x;
    text_yorigin = text_state.ref_y;
    str_ptr = instr_ptr as *const u8;
    length = instr_len;
    if (*font).format == 3i32 {
        if handle_multibyte_string(font, &mut str_ptr, &mut length, ctype) < 0i32 {
            _tt_abort(
                b"Error in converting input string...\x00" as *const u8 as *const i8,
            );
        }
        if !(*real_font).used_chars.is_null() {
            i = 0i32 as size_t;
            while i < length {
                let mut cid: u16 = ((*str_ptr.offset(i as isize) as i32) << 8i32
                    | *str_ptr.offset(i.wrapping_add(1i32 as u64) as isize)
                        as i32)
                    as u16;
                let ref mut fresh38 = *(*real_font)
                    .used_chars
                    .offset((cid as i32 / 8i32) as isize);
                *fresh38 = (*fresh38 as i32 | 1i32 << 7i32 - cid as i32 % 8i32)
                    as i8;
                i = (i as u64).wrapping_add(2i32 as u64) as size_t as size_t
            }
        }
    } else if !(*real_font).used_chars.is_null() {
        i = 0i32 as size_t;
        while i < length {
            *(*real_font)
                .used_chars
                .offset(*str_ptr.offset(i as isize) as isize) = 1i32 as i8;
            i = i.wrapping_add(1)
        }
    }
    if num_dev_coords > 0i32 {
        xpos -= round((*dev_coords.offset((num_dev_coords - 1i32) as isize)).x / dev_unit.dvi2pts)
            as spt_t;
        ypos -= round((*dev_coords.offset((num_dev_coords - 1i32) as isize)).y / dev_unit.dvi2pts)
            as spt_t
    }
    /*
     * Kern is in units of character units, i.e., 1000 = 1 em.
     *
     * Positive kern means kerning (reduce excess white space).
     *
     * The following formula is of the form a*x/b where a, x, and b are signed long
     * integers.  Since in integer arithmetic (a*x) could overflow and a*(x/b) would
     * not be accurate, we use floating point arithmetic rather than trying to do
     * this all with integer arithmetic.
     *
     * 1000.0 / (font->extend * font->sptsize) is caluculated each times...
     * Is accuracy really a matter? Character widths are always rounded to integer
     * (in 1000 units per em) but dvipdfmx does not take into account of this...
     */
    if text_state.dir_mode == 0i32 {
        /* Left-to-right */
        delh = text_xorigin + text_state.offset - xpos;
        delv = ypos - text_yorigin
    } else if text_state.dir_mode == 1i32 {
        /* Top-to-bottom */
        delh = ypos - text_yorigin + text_state.offset;
        delv = xpos - text_xorigin
    } else {
        /* Bottom-to-top */
        delh = ypos + text_yorigin + text_state.offset;
        delv = xpos + text_xorigin
    }
    /* White-space more than 3em is not considered as a part of single text.
     * So we will break string mode in that case.
     * Dvipdfmx spend most of time processing strings with kern = 0 (but far
     * more times in font handling).
     * You may want to use pre-calculated value for WORD_SPACE_MAX.
     * More text compression may be possible by replacing kern with space char
     * when -kern is equal to space char width.
     */
    if text_state.force_reset != 0
        || labs(delv as i64) > dev_unit.min_bp_val as i64
        || labs(delh as i64)
            > (3.0f64 * (*font).extend * (*font).sptsize as f64) as spt_t as i64
    {
        text_mode();
        kern = 0i32
    } else {
        kern = (1000.0f64 / (*font).extend * delh as f64
            / (*font).sptsize as f64) as spt_t
    }
    /* Inaccucary introduced by rounding of character width appears within
     * single text block. There are point_size/1000 rounding error per character.
     * If you really care about accuracy, you should compensate this here too.
     */
    if motion_state != 3i32 {
        string_mode(
            xpos,
            ypos,
            (*font).slant,
            (*font).extend,
            text_state.matrix.rotate,
        );
    } else if kern != 0i32 {
        /*
         * Same issues as earlier. Use floating point for simplicity.
         * This routine needs to be fast, so we don't call sprintf() or strcpy().
         */
        text_state.offset -= (kern as f64
            * (*font).extend
            * ((*font).sptsize as f64 / 1000.0f64))
            as spt_t; /* op: */
        let fresh39 = len;
        len = len.wrapping_add(1);
        format_buffer[fresh39 as usize] = (if text_state.is_mb != 0 {
            '>' as i32
        } else {
            ')' as i32
        }) as i8;
        if (*font).wmode != 0 {
            len = (len as u64)
                .wrapping_add(
                    p_itoa(-kern, format_buffer.as_mut_ptr().offset(len as isize)) as u64,
                ) as size_t as size_t
        } else {
            len = (len as u64)
                .wrapping_add(
                    p_itoa(kern, format_buffer.as_mut_ptr().offset(len as isize)) as u64,
                ) as size_t as size_t
        }
        let fresh40 = len;
        len = len.wrapping_add(1);
        format_buffer[fresh40 as usize] = (if text_state.is_mb != 0 {
            '<' as i32
        } else {
            '(' as i32
        }) as i8;
        pdf_doc_add_page_content(format_buffer.as_mut_ptr(), len as libc::c_uint);
        len = 0i32 as size_t
    }
    if text_state.is_mb != 0 {
        if (4096i32 as u64).wrapping_sub(len)
            < (2i32 as u64).wrapping_mul(length)
        {
            _tt_abort(b"Buffer overflow...\x00" as *const u8 as *const i8);
        }
        i = 0i32 as size_t;
        while i < length {
            let mut first: i32 = 0;
            let mut second: i32 = 0;
            first = *str_ptr.offset(i as isize) as i32 >> 4i32 & 0xfi32;
            second = *str_ptr.offset(i as isize) as i32 & 0xfi32;
            let fresh41 = len;
            len = len.wrapping_add(1);
            format_buffer[fresh41 as usize] = (if first >= 10i32 {
                first + 'W' as i32
            } else {
                first + '0' as i32
            }) as i8;
            let fresh42 = len;
            len = len.wrapping_add(1);
            format_buffer[fresh42 as usize] = (if second >= 10i32 {
                second + 'W' as i32
            } else {
                second + '0' as i32
            }) as i8;
            i = i.wrapping_add(1)
        }
    } else {
        len = (len as u64).wrapping_add(pdfobj_escape_str(
            format_buffer.as_mut_ptr().offset(len as isize),
            (4096i32 as u64).wrapping_sub(len),
            str_ptr,
            length,
        )) as size_t as size_t
    }
    /* I think if you really care about speed, you should avoid memcopy here. */
    pdf_doc_add_page_content(format_buffer.as_mut_ptr(), len as libc::c_uint); /* op: */
    text_state.offset += width;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_init_device(
    mut dvi2pts: f64,
    mut precision: i32,
    mut black_and_white: i32,
) {
    if precision < 0i32 || precision > 8i32 {
        dpx_warning(
            b"Number of decimal digits out of range [0-%d].\x00" as *const u8
                as *const i8,
            8i32,
        );
    }
    if precision < 0i32 {
        dev_unit.precision = 0i32
    } else if precision > 8i32 {
        dev_unit.precision = 8i32
    } else {
        dev_unit.precision = precision
    }
    dev_unit.dvi2pts = dvi2pts;
    dev_unit.min_bp_val = (floor(
        1.0f64
            / (ten_pow[dev_unit.precision as usize] as f64 * dvi2pts)
            / 1i32 as f64
            + 0.5f64,
    ) * 1i32 as f64) as i32;
    if dev_unit.min_bp_val < 0i32 {
        dev_unit.min_bp_val = -dev_unit.min_bp_val
    }
    dev_param.colormode = if black_and_white != 0 { 0i32 } else { 1i32 };
    graphics_mode();
    pdf_color_clear_stack();
    pdf_dev_init_gstates();
    max_dev_fonts = 0i32;
    num_dev_fonts = max_dev_fonts;
    dev_fonts = 0 as *mut dev_font;
    max_dev_coords = 0i32;
    num_dev_coords = max_dev_coords;
    dev_coords = 0 as *mut pdf_coord;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_close_device() {
    if !dev_fonts.is_null() {
        let mut i: i32 = 0;
        i = 0i32;
        while i < num_dev_fonts {
            free((*dev_fonts.offset(i as isize)).tex_name as *mut libc::c_void);
            pdf_release_obj((*dev_fonts.offset(i as isize)).resource);
            let ref mut fresh43 = (*dev_fonts.offset(i as isize)).tex_name;
            *fresh43 = 0 as *mut i8;
            let ref mut fresh44 = (*dev_fonts.offset(i as isize)).resource;
            *fresh44 = 0 as *mut pdf_obj;
            let ref mut fresh45 = (*dev_fonts.offset(i as isize)).cff_charsets;
            *fresh45 = 0 as *mut cff_charsets;
            i += 1
        }
        free(dev_fonts as *mut libc::c_void);
    }
    free(dev_coords as *mut libc::c_void);
    pdf_dev_clear_gstates();
}
/*
 * BOP, EOP, and FONT section.
 * BOP and EOP manipulate some of the same data structures
 * as the font stuff.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_reset_fonts(mut newpage: i32) {
    let mut i: i32 = 0;
    i = 0i32;
    while i < num_dev_fonts {
        (*dev_fonts.offset(i as isize)).used_on_this_page = 0i32;
        i += 1
    }
    text_state.font_id = -1i32;
    text_state.matrix.slant = 0.0f64;
    text_state.matrix.extend = 1.0f64;
    text_state.matrix.rotate = 0i32;
    if newpage != 0 {
        text_state.bold_param = 0.0f64
    }
    text_state.is_mb = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_reset_color(mut force: i32) {
    let mut sc: *mut pdf_color = 0 as *mut pdf_color;
    let mut fc: *mut pdf_color = 0 as *mut pdf_color;
    pdf_color_get_current(&mut sc, &mut fc);
    pdf_dev_set_color(sc, 0i32 as i8, force);
    pdf_dev_set_color(fc, 0x20i32 as i8, force);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_bop(mut M: *const pdf_tmatrix) {
    graphics_mode();
    text_state.force_reset = 0i32;
    pdf_dev_gsave();
    pdf_dev_concat(M);
    pdf_dev_reset_fonts(1i32);
    pdf_dev_reset_color(0i32);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_eop() {
    let mut depth: i32 = 0;
    graphics_mode();
    depth = pdf_dev_current_depth();
    if depth != 1i32 {
        dpx_warning(
            b"Unbalenced q/Q nesting...: %d\x00" as *const u8 as *const i8,
            depth,
        );
        pdf_dev_grestore_to(0i32);
    } else {
        pdf_dev_grestore();
    };
}
unsafe extern "C" fn print_fontmap(mut font_name: *const i8, mut mrec: *mut fontmap_rec) {
    if mrec.is_null() {
        return;
    }
    dpx_message(b"\n\x00" as *const u8 as *const i8);
    dpx_message(
        b"fontmap: %s -> %s\x00" as *const u8 as *const i8,
        font_name,
        (*mrec).font_name,
    );
    if !(*mrec).enc_name.is_null() {
        dpx_message(
            b"(%s)\x00" as *const u8 as *const i8,
            (*mrec).enc_name,
        );
    }
    if (*mrec).opt.extend != 1.0f64 {
        dpx_message(
            b"[extend:%g]\x00" as *const u8 as *const i8,
            (*mrec).opt.extend,
        );
    }
    if (*mrec).opt.slant != 0.0f64 {
        dpx_message(
            b"[slant:%g]\x00" as *const u8 as *const i8,
            (*mrec).opt.slant,
        );
    }
    if (*mrec).opt.bold != 0.0f64 {
        dpx_message(
            b"[bold:%g]\x00" as *const u8 as *const i8,
            (*mrec).opt.bold,
        );
    }
    if (*mrec).opt.flags & 1i32 << 1i32 != 0 {
        dpx_message(b"[noemb]\x00" as *const u8 as *const i8);
    }
    if (*mrec).opt.mapc >= 0i32 {
        dpx_message(
            b"[map:<%02x>]\x00" as *const u8 as *const i8,
            (*mrec).opt.mapc,
        );
    }
    if !(*mrec).opt.charcoll.is_null() {
        dpx_message(
            b"[csi:%s]\x00" as *const u8 as *const i8,
            (*mrec).opt.charcoll,
        );
    }
    if (*mrec).opt.index != 0 {
        dpx_message(
            b"[index:%d]\x00" as *const u8 as *const i8,
            (*mrec).opt.index,
        );
    }
    match (*mrec).opt.style {
        1 => {
            dpx_message(b"[style:bold]\x00" as *const u8 as *const i8);
        }
        2 => {
            dpx_message(b"[style:italic]\x00" as *const u8 as *const i8);
        }
        3 => {
            dpx_message(b"[style:bolditalic]\x00" as *const u8 as *const i8);
        }
        _ => {}
    }
    dpx_message(b"\n\x00" as *const u8 as *const i8);
}
/* _FIXME_
 * Font is identified with font_name and point_size as in DVI here.
 * However, except for PDF_FONTTYPE_BITMAP, we can share the
 * short_name, resource and used_chars between multiple instances
 * of the same font at different sizes.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_locate_font(
    mut font_name: *const i8,
    mut ptsize: spt_t,
) -> i32 {
    let mut i: i32 = 0; /* found a dev_font that matches the request */
    let mut mrec: *mut fontmap_rec = 0 as *mut fontmap_rec;
    let mut font: *mut dev_font = 0 as *mut dev_font;
    if font_name.is_null() {
        return -1i32;
    }
    if ptsize == 0i32 {
        _tt_abort(
            b"pdf_dev_locate_font() called with the zero ptsize.\x00" as *const u8
                as *const i8,
        );
    }
    i = 0i32;
    while i < num_dev_fonts {
        if streq_ptr(font_name, (*dev_fonts.offset(i as isize)).tex_name) {
            if ptsize == (*dev_fonts.offset(i as isize)).sptsize {
                return i;
            }
            if (*dev_fonts.offset(i as isize)).format != 2i32 {
                break;
            }
            /* new dev_font will share pdf resource with /i/ */
        }
        i += 1
    }
    /*
     * Make sure we have room for a new one, even though we may not
     * actually create one.
     */
    if num_dev_fonts >= max_dev_fonts {
        max_dev_fonts += 16i32;
        dev_fonts = renew(
            dev_fonts as *mut libc::c_void,
            (max_dev_fonts as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<dev_font>() as u64)
                as u32,
        ) as *mut dev_font
    }
    font = &mut *dev_fonts.offset(num_dev_fonts as isize) as *mut dev_font;
    /* New font */
    mrec = pdf_lookup_fontmap_record(font_name);
    if verbose > 1i32 {
        print_fontmap(font_name, mrec);
    }
    (*font).font_id =
        pdf_font_findresource(font_name, ptsize as f64 * dev_unit.dvi2pts, mrec);
    if (*font).font_id < 0i32 {
        return -1i32;
    }
    if !mrec.is_null() {
        (*font).cff_charsets = (*mrec).opt.cff_charsets as *mut cff_charsets
    }
    /* We found device font here. */
    if i < num_dev_fonts {
        (*font).real_font_index = i; /* NULL terminated here */
        strcpy(
            (*font).short_name.as_mut_ptr(),
            (*dev_fonts.offset(i as isize)).short_name.as_mut_ptr(),
        ); /* Don't ref obj until font is actually used. */
    } else {
        (*font).real_font_index = -1i32;
        (*font).short_name[0] = 'F' as i32 as i8;
        p_itoa(
            num_phys_fonts + 1i32,
            &mut *(*font).short_name.as_mut_ptr().offset(1),
        );
        num_phys_fonts += 1
    }
    (*font).used_on_this_page = 0i32;
    (*font).tex_name = new(
        (strlen(font_name).wrapping_add(1i32 as u64) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32,
    ) as *mut i8;
    strcpy((*font).tex_name, font_name);
    (*font).sptsize = ptsize;
    match pdf_get_font_subtype((*font).font_id) {
        2 => (*font).format = 2i32,
        4 => (*font).format = 3i32,
        _ => (*font).format = 1i32,
    }
    (*font).wmode = pdf_get_font_wmode((*font).font_id);
    (*font).enc_id = pdf_get_font_encoding((*font).font_id);
    (*font).resource = 0 as *mut pdf_obj;
    (*font).used_chars = 0 as *mut i8;
    (*font).extend = 1.0f64;
    (*font).slant = 0.0f64;
    (*font).bold = 0.0f64;
    (*font).mapc = -1i32;
    (*font).is_unicode = 0i32;
    (*font).ucs_group = 0i32;
    (*font).ucs_plane = 0i32;
    if !mrec.is_null() {
        (*font).extend = (*mrec).opt.extend;
        (*font).slant = (*mrec).opt.slant;
        (*font).bold = (*mrec).opt.bold;
        if (*mrec).opt.mapc >= 0i32 {
            (*font).mapc = (*mrec).opt.mapc >> 8i32 & 0xffi32
        } else {
            (*font).mapc = -1i32
        }
        if streq_ptr(
            (*mrec).enc_name,
            b"unicode\x00" as *const u8 as *const i8,
        ) {
            (*font).is_unicode = 1i32;
            if (*mrec).opt.mapc >= 0i32 {
                (*font).ucs_group = (*mrec).opt.mapc >> 24i32 & 0xffi32;
                (*font).ucs_plane = (*mrec).opt.mapc >> 16i32 & 0xffi32
            } else {
                (*font).ucs_group = 0i32;
                (*font).ucs_plane = 0i32
            }
        } else {
            (*font).is_unicode = 0i32
        }
    }
    let fresh46 = num_dev_fonts;
    num_dev_fonts = num_dev_fonts + 1;
    return fresh46;
}
/* This does not remember current stroking width. */
unsafe extern "C" fn dev_sprint_line(
    mut buf: *mut i8,
    mut width: spt_t,
    mut p0_x: spt_t,
    mut p0_y: spt_t,
    mut p1_x: spt_t,
    mut p1_y: spt_t,
) -> i32 {
    let mut len: i32 = 0i32;
    let mut w: f64 = 0.;
    w = width as f64 * dev_unit.dvi2pts;
    len += p_dtoa(
        w,
        if dev_unit.precision + 1i32 < 8i32 {
            dev_unit.precision + 1i32
        } else {
            8i32
        },
        buf.offset(len as isize),
    );
    let fresh47 = len;
    len = len + 1;
    *buf.offset(fresh47 as isize) = ' ' as i32 as i8;
    let fresh48 = len;
    len = len + 1;
    *buf.offset(fresh48 as isize) = 'w' as i32 as i8;
    let fresh49 = len;
    len = len + 1;
    *buf.offset(fresh49 as isize) = ' ' as i32 as i8;
    len += dev_sprint_bp(buf.offset(len as isize), p0_x, 0 as *mut spt_t);
    let fresh50 = len;
    len = len + 1;
    *buf.offset(fresh50 as isize) = ' ' as i32 as i8;
    len += dev_sprint_bp(buf.offset(len as isize), p0_y, 0 as *mut spt_t);
    let fresh51 = len;
    len = len + 1;
    *buf.offset(fresh51 as isize) = ' ' as i32 as i8;
    let fresh52 = len;
    len = len + 1;
    *buf.offset(fresh52 as isize) = 'm' as i32 as i8;
    let fresh53 = len;
    len = len + 1;
    *buf.offset(fresh53 as isize) = ' ' as i32 as i8;
    len += dev_sprint_bp(buf.offset(len as isize), p1_x, 0 as *mut spt_t);
    let fresh54 = len;
    len = len + 1;
    *buf.offset(fresh54 as isize) = ' ' as i32 as i8;
    len += dev_sprint_bp(buf.offset(len as isize), p1_y, 0 as *mut spt_t);
    let fresh55 = len;
    len = len + 1;
    *buf.offset(fresh55 as isize) = ' ' as i32 as i8;
    let fresh56 = len;
    len = len + 1;
    *buf.offset(fresh56 as isize) = 'l' as i32 as i8;
    let fresh57 = len;
    len = len + 1;
    *buf.offset(fresh57 as isize) = ' ' as i32 as i8;
    let fresh58 = len;
    len = len + 1;
    *buf.offset(fresh58 as isize) = 'S' as i32 as i8;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_set_rule(
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut width: spt_t,
    mut height: spt_t,
) {
    let mut len: i32 = 0i32;
    let mut width_in_bp: f64 = 0.;
    if num_dev_coords > 0i32 {
        xpos -= round((*dev_coords.offset((num_dev_coords - 1i32) as isize)).x / dev_unit.dvi2pts)
            as spt_t;
        ypos -= round((*dev_coords.offset((num_dev_coords - 1i32) as isize)).y / dev_unit.dvi2pts)
            as spt_t
    }
    graphics_mode();
    let fresh59 = len;
    len = len + 1;
    format_buffer[fresh59 as usize] = ' ' as i32 as i8;
    let fresh60 = len;
    len = len + 1;
    format_buffer[fresh60 as usize] = 'q' as i32 as i8;
    let fresh61 = len;
    len = len + 1;
    format_buffer[fresh61 as usize] = ' ' as i32 as i8;
    /* Don't use too thick line. */
    width_in_bp =
        (if width < height { width } else { height }) as f64 * dev_unit.dvi2pts;
    if width_in_bp < 0.0f64 || width_in_bp > 5.0f64 {
        let mut rect: pdf_rect = pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        };
        rect.llx = dev_unit.dvi2pts * xpos as f64;
        rect.lly = dev_unit.dvi2pts * ypos as f64;
        rect.urx = dev_unit.dvi2pts * width as f64;
        rect.ury = dev_unit.dvi2pts * height as f64;
        len += pdf_sprint_rect(format_buffer.as_mut_ptr().offset(len as isize), &mut rect);
        let fresh62 = len;
        len = len + 1;
        format_buffer[fresh62 as usize] = ' ' as i32 as i8;
        let fresh63 = len;
        len = len + 1;
        format_buffer[fresh63 as usize] = 'r' as i32 as i8;
        let fresh64 = len;
        len = len + 1;
        format_buffer[fresh64 as usize] = 'e' as i32 as i8;
        let fresh65 = len;
        len = len + 1;
        format_buffer[fresh65 as usize] = ' ' as i32 as i8;
        let fresh66 = len;
        len = len + 1;
        format_buffer[fresh66 as usize] = 'f' as i32 as i8
    } else if width > height {
        /* NOTE:
         *  A line width of 0 denotes the thinnest line that can be rendered at
         *  device resolution. See, PDF Reference Manual 4th ed., sec. 4.3.2,
         *  "Details of Graphics State Parameters", p. 185.
         */
        if height < dev_unit.min_bp_val {
            dpx_warning(
                b"Too thin line: height=%d (%g bp)\x00" as *const u8 as *const i8,
                height,
                width_in_bp,
            );
            dpx_warning(
                b"Please consider using \"-d\" option.\x00" as *const u8 as *const i8,
            );
        }
        len += dev_sprint_line(
            format_buffer.as_mut_ptr().offset(len as isize),
            height,
            xpos,
            ypos + height / 2i32,
            xpos + width,
            ypos + height / 2i32,
        )
    } else {
        if width < dev_unit.min_bp_val {
            dpx_warning(
                b"Too thin line: width=%d (%g bp)\x00" as *const u8 as *const i8,
                width,
                width_in_bp,
            );
            dpx_warning(
                b"Please consider using \"-d\" option.\x00" as *const u8 as *const i8,
            );
        }
        len += dev_sprint_line(
            format_buffer.as_mut_ptr().offset(len as isize),
            width,
            xpos + width / 2i32,
            ypos,
            xpos + width / 2i32,
            ypos + height,
        )
    }
    let fresh67 = len;
    len = len + 1;
    format_buffer[fresh67 as usize] = ' ' as i32 as i8;
    let fresh68 = len;
    len = len + 1;
    format_buffer[fresh68 as usize] = 'Q' as i32 as i8;
    pdf_doc_add_page_content(format_buffer.as_mut_ptr(), len as libc::c_uint);
    /* op: q re f Q */
}
/* Rectangle in device space coordinate. */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_set_rect(
    mut rect: *mut pdf_rect,
    mut x_user: spt_t,
    mut y_user: spt_t,
    mut width: spt_t,
    mut height: spt_t,
    mut depth: spt_t,
) {
    let mut dev_x: f64 = 0.; /* currentmatrix */
    let mut dev_y: f64 = 0.; /* 0 for B&W */
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p3: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut min_x: f64 = 0.;
    let mut min_y: f64 = 0.;
    let mut max_x: f64 = 0.;
    let mut max_y: f64 = 0.;
    dev_x = x_user as f64 * dev_unit.dvi2pts;
    dev_y = y_user as f64 * dev_unit.dvi2pts;
    if text_state.dir_mode != 0 {
        p0.x = dev_x - dev_unit.dvi2pts * depth as f64;
        p0.y = dev_y - dev_unit.dvi2pts * width as f64;
        p1.x = dev_x + dev_unit.dvi2pts * height as f64;
        p1.y = p0.y;
        p2.x = p1.x;
        p2.y = dev_y;
        p3.x = p0.x;
        p3.y = p2.y
    } else {
        p0.x = dev_x;
        p0.y = dev_y - dev_unit.dvi2pts * depth as f64;
        p1.x = dev_x + dev_unit.dvi2pts * width as f64;
        p1.y = p0.y;
        p2.x = p1.x;
        p2.y = dev_y + dev_unit.dvi2pts * height as f64;
        p3.x = p0.x;
        p3.y = p2.y
    }
    pdf_dev_transform(&mut p0, 0 as *const pdf_tmatrix);
    pdf_dev_transform(&mut p1, 0 as *const pdf_tmatrix);
    pdf_dev_transform(&mut p2, 0 as *const pdf_tmatrix);
    pdf_dev_transform(&mut p3, 0 as *const pdf_tmatrix);
    min_x = if p0.x < p1.x { p0.x } else { p1.x };
    min_x = if min_x < p2.x { min_x } else { p2.x };
    min_x = if min_x < p3.x { min_x } else { p3.x };
    max_x = if p0.x > p1.x { p0.x } else { p1.x };
    max_x = if max_x > p2.x { max_x } else { p2.x };
    max_x = if max_x > p3.x { max_x } else { p3.x };
    min_y = if p0.y < p1.y { p0.y } else { p1.y };
    min_y = if min_y < p2.y { min_y } else { p2.y };
    min_y = if min_y < p3.y { min_y } else { p3.y };
    max_y = if p0.y > p1.y { p0.y } else { p1.y };
    max_y = if max_y > p2.y { max_y } else { p2.y };
    max_y = if max_y > p3.y { max_y } else { p3.y };
    (*rect).llx = min_x;
    (*rect).lly = min_y;
    (*rect).urx = max_x;
    (*rect).ury = max_y;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_get_dirmode() -> i32 {
    return text_state.dir_mode;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_set_dirmode(mut text_dir: i32) {
    let mut font: *mut dev_font = 0 as *mut dev_font;
    let mut text_rotate: i32 = 0;
    let mut vert_dir: i32 = 0;
    let mut vert_font: i32 = 0;
    font = if text_state.font_id < 0i32 {
        0 as *mut dev_font
    } else {
        &mut *dev_fonts.offset(text_state.font_id as isize) as *mut dev_font
    };
    vert_font = if !font.is_null() && (*font).wmode != 0 {
        1i32
    } else {
        0i32
    };
    if dev_param.autorotate != 0 {
        vert_dir = text_dir
    } else {
        vert_dir = vert_font
    }
    text_rotate = vert_font << 2i32 | vert_dir;
    if !font.is_null()
        && (if abs(text_rotate - text_state.matrix.rotate) % 5i32 == 0i32 {
            0i32
        } else {
            1i32
        }) != 0
    {
        text_state.force_reset = 1i32
    }
    text_state.matrix.rotate = text_rotate;
    text_state.dir_mode = text_dir;
}
unsafe extern "C" fn dev_set_param_autorotate(mut auto_rotate: i32) {
    let mut font: *mut dev_font = 0 as *mut dev_font;
    let mut text_rotate: i32 = 0;
    let mut vert_font: i32 = 0;
    let mut vert_dir: i32 = 0;
    font = if text_state.font_id < 0i32 {
        0 as *mut dev_font
    } else {
        &mut *dev_fonts.offset(text_state.font_id as isize) as *mut dev_font
    };
    vert_font = if !font.is_null() && (*font).wmode != 0 {
        1i32
    } else {
        0i32
    };
    if auto_rotate != 0 {
        vert_dir = text_state.dir_mode
    } else {
        vert_dir = vert_font
    }
    text_rotate = vert_font << 2i32 | vert_dir;
    if if abs(text_rotate - text_state.matrix.rotate) % 5i32 == 0i32 {
        0i32
    } else {
        1i32
    } != 0
    {
        text_state.force_reset = 1i32
    }
    text_state.matrix.rotate = text_rotate;
    dev_param.autorotate = auto_rotate;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_get_param(mut param_type: i32) -> i32 {
    let mut value: i32 = 0i32;
    match param_type {
        1 => value = dev_param.autorotate,
        2 => value = dev_param.colormode,
        _ => {
            _tt_abort(
                b"Unknown device parameter: %d\x00" as *const u8 as *const i8,
                param_type,
            );
        }
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_set_param(mut param_type: i32, mut value: i32) {
    match param_type {
        1 => {
            dev_set_param_autorotate(value);
        }
        2 => dev_param.colormode = value,
        _ => {
            _tt_abort(
                b"Unknown device parameter: %d\x00" as *const u8 as *const i8,
                param_type,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_put_image(
    mut id: i32,
    mut p: *mut transform_info,
    mut ref_x: f64,
    mut ref_y: f64,
) -> i32 {
    let mut res_name: *mut i8 = 0 as *mut i8;
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut M1: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut len: i32 = 0i32;
    if num_dev_coords > 0i32 {
        ref_x -= (*dev_coords.offset((num_dev_coords - 1i32) as isize)).x;
        ref_y -= (*dev_coords.offset((num_dev_coords - 1i32) as isize)).y
    }
    M.a = (*p).matrix.a;
    M.b = (*p).matrix.b;
    M.c = (*p).matrix.c;
    M.d = (*p).matrix.d;
    M.e = (*p).matrix.e;
    M.f = (*p).matrix.f;
    M.e += ref_x;
    M.f += ref_y;
    /* Just rotate by -90, but not tested yet. Any problem if M has scaling? */
    if dev_param.autorotate != 0 && text_state.dir_mode != 0 {
        let mut tmp: f64 = 0.;
        tmp = -M.a;
        M.a = M.b;
        M.b = tmp;
        tmp = -M.c;
        M.c = M.d;
        M.d = tmp
    }
    graphics_mode();
    pdf_dev_gsave();
    pdf_ximage_scale_image(id, &mut M1, &mut r, p);
    let mut _tmp_a: f64 = 0.;
    let mut _tmp_b: f64 = 0.;
    let mut _tmp_c: f64 = 0.;
    let mut _tmp_d: f64 = 0.;
    _tmp_a = M.a;
    _tmp_b = M.b;
    _tmp_c = M.c;
    _tmp_d = M.d;
    M.a = M1.a * _tmp_a + M1.b * _tmp_c;
    M.b = M1.a * _tmp_b + M1.b * _tmp_d;
    M.c = M1.c * _tmp_a + M1.d * _tmp_c;
    M.d = M1.c * _tmp_b + M1.d * _tmp_d;
    M.e += M1.e * _tmp_a + M1.f * _tmp_c;
    M.f += M1.e * _tmp_b + M1.f * _tmp_d;
    pdf_dev_concat(&mut M);
    /* Clip */
    if (*p).flags & 1i32 << 3i32 != 0 {
        pdf_dev_rectclip(r.llx, r.lly, r.urx - r.llx, r.ury - r.lly); /* op: Do */
    }
    res_name = pdf_ximage_get_resname(id);
    len = sprintf(
        work_buffer.as_mut_ptr(),
        b" /%s Do\x00" as *const u8 as *const i8,
        res_name,
    );
    pdf_doc_add_page_content(work_buffer.as_mut_ptr(), len as libc::c_uint);
    pdf_dev_grestore();
    pdf_doc_add_page_resource(
        b"XObject\x00" as *const u8 as *const i8,
        res_name,
        pdf_ximage_get_reference(id),
    );
    if dvi_is_tracking_boxes() {
        let mut P: pdf_tmatrix = pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        };
        let mut i: libc::c_uint = 0;
        let mut rect: pdf_rect = pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        };
        let mut corner: [pdf_coord; 4] = [pdf_coord { x: 0., y: 0. }; 4];
        pdf_dev_set_rect(
            &mut rect,
            (65536i32 as f64 * ref_x) as spt_t,
            (65536i32 as f64 * ref_y) as spt_t,
            (65536i32 as f64 * (r.urx - r.llx)) as spt_t,
            (65536i32 as f64 * (r.ury - r.lly)) as spt_t,
            0i32,
        );
        corner[0].x = rect.llx;
        corner[0].y = rect.lly;
        corner[1].x = rect.llx;
        corner[1].y = rect.ury;
        corner[2].x = rect.urx;
        corner[2].y = rect.ury;
        corner[3].x = rect.urx;
        corner[3].y = rect.lly;
        P.a = (*p).matrix.a;
        P.b = (*p).matrix.b;
        P.c = (*p).matrix.c;
        P.d = (*p).matrix.d;
        P.e = (*p).matrix.e;
        P.f = (*p).matrix.f;
        i = 0i32 as libc::c_uint;
        while i < 4i32 as libc::c_uint {
            corner[i as usize].x -= rect.llx;
            corner[i as usize].y -= rect.lly;
            pdf_dev_transform(&mut *corner.as_mut_ptr().offset(i as isize), &mut P);
            corner[i as usize].x += rect.llx;
            corner[i as usize].y += rect.lly;
            i = i.wrapping_add(1)
        }
        rect.llx = corner[0].x;
        rect.lly = corner[0].y;
        rect.urx = corner[0].x;
        rect.ury = corner[0].y;
        i = 0i32 as libc::c_uint;
        while i < 4i32 as libc::c_uint {
            if corner[i as usize].x < rect.llx {
                rect.llx = corner[i as usize].x
            }
            if corner[i as usize].x > rect.urx {
                rect.urx = corner[i as usize].x
            }
            if corner[i as usize].y < rect.lly {
                rect.lly = corner[i as usize].y
            }
            if corner[i as usize].y > rect.ury {
                rect.ury = corner[i as usize].y
            }
            i = i.wrapping_add(1)
        }
        pdf_doc_expand_box(&mut rect);
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn transform_info_clear(mut info: *mut transform_info) {
    /* Physical dimensions */
    (*info).width = 0.0f64;
    (*info).height = 0.0f64;
    (*info).depth = 0.0f64;
    (*info).bbox.llx = 0.0f64;
    (*info).bbox.lly = 0.0f64;
    (*info).bbox.urx = 0.0f64;
    (*info).bbox.ury = 0.0f64;
    /* Transformation matrix */
    (*info).matrix.a = 1.0f64;
    (*info).matrix.b = 0.0f64;
    (*info).matrix.c = 0.0f64;
    (*info).matrix.d = 1.0f64;
    (*info).matrix.e = 0.0f64;
    (*info).matrix.f = 0.0f64;
    (*info).flags = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_begin_actualtext(
    mut unicodes: *mut u16,
    mut count: i32,
) {
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut pdf_doc_enc: i32 = 1i32;
    /* check whether we can use PDFDocEncoding for this string
    (we punt on the 0x80..0xA0 range that does not directly correspond to unicode)  */
    i = 0i32; /* if using PDFDocEncoding, we only care about the low 8 bits,
              so start with the second byte of our pair */
    while i < count {
        if *unicodes.offset(i as isize) as i32 > 0xffi32
            || *unicodes.offset(i as isize) as i32 > 0x7fi32
                && (*unicodes.offset(i as isize) as i32) < 0xa1i32
        {
            pdf_doc_enc = 0i32;
            break;
        } else {
            i += 1
        }
    }
    graphics_mode();
    len = sprintf(
        work_buffer.as_mut_ptr(),
        b"\n/Span<</ActualText(\x00" as *const u8 as *const i8,
    );
    if pdf_doc_enc == 0 {
        len += sprintf(
            work_buffer.as_mut_ptr().offset(len as isize),
            b"\xfe\xff\x00" as *const u8 as *const i8,
        )
    }
    pdf_doc_add_page_content(work_buffer.as_mut_ptr(), len as libc::c_uint);
    loop {
        let fresh69 = count;
        count = count - 1;
        if !(fresh69 > 0i32) {
            break;
        }
        let mut s: [u8; 2] = [
            (*unicodes as i32 >> 8i32) as u8,
            (*unicodes as i32 & 0xffi32) as u8,
        ];
        i = pdf_doc_enc;
        len = 0i32;
        while i < 2i32 {
            let mut c: u8 = s[i as usize];
            if c as i32 == '(' as i32
                || c as i32 == ')' as i32
                || c as i32 == '\\' as i32
            {
                len += sprintf(
                    work_buffer.as_mut_ptr().offset(len as isize),
                    b"\\%c\x00" as *const u8 as *const i8,
                    c as i32,
                )
            } else if (c as i32) < ' ' as i32 {
                len += sprintf(
                    work_buffer.as_mut_ptr().offset(len as isize),
                    b"\\%03o\x00" as *const u8 as *const i8,
                    c as i32,
                )
            } else {
                len += sprintf(
                    work_buffer.as_mut_ptr().offset(len as isize),
                    b"%c\x00" as *const u8 as *const i8,
                    c as i32,
                )
            }
            i += 1
        }
        pdf_doc_add_page_content(work_buffer.as_mut_ptr(), len as libc::c_uint);
        unicodes = unicodes.offset(1)
    }
    len = sprintf(
        work_buffer.as_mut_ptr(),
        b")>>BDC\x00" as *const u8 as *const i8,
    );
    pdf_doc_add_page_content(work_buffer.as_mut_ptr(), len as libc::c_uint);
}
/* Not in spt_t. */
/* unit_conv: multiplier for input unit (spt_t) to bp conversion.
 * precision: How many fractional digits preserved in output (not real
 *            accuracy control).
 * is_bw:     Ignore color related special instructions.
 */
/* returns 1.0/unit_conv */
/* Draw texts and rules:
 *
 * xpos, ypos, width, and height are all fixed-point numbers
 * converted to big-points by multiplying unit_conv (dvi2pts).
 * They must be position in the user space.
 *
 * ctype:
 *   0 - input string is in multi-byte encoding.
 *   1 - input string is in 8-bit encoding.
 *   2 - input string is in 16-bit encoding.
 */
/* Place XObject */
/* The design_size and ptsize required by PK font support...
 */
/* The following two routines are NOT WORKING.
 * Dvipdfmx doesn't manage gstate well..
 */
/* Always returns 1.0, please rename this. */
/* Access text state parameters. */
/* ps: special support want this (pTeX). */
/* Text composition (direction) mode
 * This affects only when auto_rotate is enabled.
 */
/* Set rect to rectangle in device space.
 * Unit conversion spt_t to bp and transformation applied within it.
 */
/* Accessor to various device parameters.
 */
/* Text composition mode is ignored (always same as font's
 * writing mode) and glyph rotation is not enabled if
 * auto_rotate is unset.
 */
/*
 * For pdf_doc, pdf_draw and others.
 */
/* Force reselecting font and color:
 * XFrom (content grabbing) and Metapost support want them.
 */
/* Initialization of transformation matrix with M and others.
 * They are called within pdf_doc_begin_page() and pdf_doc_end_page().
 */
/* Text is normal and line art is not normal in dvipdfmx. So we don't have
 * begin_text (BT in PDF) and end_text (ET), but instead we have graphics_mode()
 * to terminate text section. pdf_dev_flushpath() and others call this.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_end_actualtext() {
    graphics_mode();
    pdf_doc_add_page_content(
        b" EMC\x00" as *const u8 as *const i8,
        4i32 as libc::c_uint,
    );
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
/* The name transform_info is misleading.
 * I'll put this here for a moment...
 */
/* Physical dimensions
 *
 * If those values are given, images will be scaled
 * and/or shifted to fit within a box described by
 * those values.
 */
/* transform matrix */
/* user_bbox */
#[no_mangle]
pub unsafe extern "C" fn pdf_dev_reset_global_state() {
    dev_fonts = 0 as *mut dev_font;
    num_dev_fonts = 0i32;
    max_dev_fonts = 0i32;
    num_phys_fonts = 0i32;
}
