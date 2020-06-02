/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::GPUBindGroupLayoutBinding::{
    GPUBindGroupLayoutEntry, GPUBindGroupLayoutMethods,
};
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use dom_struct::dom_struct;
use std::cell::Cell;
use std::collections::HashMap;
use webgpu::{WebGPU, WebGPUBindGroupLayout};

#[dom_struct]
pub struct GPUBindGroupLayout {
    reflector_: Reflector,
    label: DomRefCell<Option<DOMString>>,
    bind_group_layout: WebGPUBindGroupLayout,
    #[ignore_malloc_size_of = "defined in webgpu"]
    entry_map: HashMap<u32, GPUBindGroupLayoutEntry>,
    #[ignore_malloc_size_of = "defined in webgpu"]
    channel: WebGPU,
    valid: Cell<bool>,
}

impl GPUBindGroupLayout {
    fn new_inherited(
        channel: WebGPU,
        bind_group_layout: WebGPUBindGroupLayout,
        entry_map: HashMap<u32, GPUBindGroupLayoutEntry>,
        valid: bool,
    ) -> Self {
        Self {
            reflector_: Reflector::new(),
            channel,
            label: DomRefCell::new(None),
            bind_group_layout,
            entry_map,
            valid: Cell::new(valid),
        }
    }

    pub fn new(
        global: &GlobalScope,
        channel: WebGPU,
        bind_group_layout: WebGPUBindGroupLayout,
        entry_map: HashMap<u32, GPUBindGroupLayoutEntry>,
        valid: bool,
    ) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(GPUBindGroupLayout::new_inherited(
                channel,
                bind_group_layout,
                entry_map,
                valid,
            )),
            global,
        )
    }
}

impl GPUBindGroupLayout {
    pub fn is_valid(&self) -> bool {
        self.valid.get()
    }

    pub fn id(&self) -> WebGPUBindGroupLayout {
        self.bind_group_layout
    }

    pub fn entries(&self) -> &HashMap<u32, GPUBindGroupLayoutEntry> {
        &self.entry_map
    }
}

impl GPUBindGroupLayoutMethods for GPUBindGroupLayout {
    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn GetLabel(&self) -> Option<DOMString> {
        self.label.borrow().clone()
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn SetLabel(&self, value: Option<DOMString>) {
        *self.label.borrow_mut() = value;
    }
}
