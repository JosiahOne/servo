/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::GPUPipelineLayoutBinding::GPUPipelineLayoutMethods;
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::globalscope::GlobalScope;
use dom_struct::dom_struct;
use std::cell::Cell;
use webgpu::WebGPUPipelineLayout;

#[dom_struct]
pub struct GPUPipelineLayout {
    reflector_: Reflector,
    label: DomRefCell<Option<DOMString>>,
    pipeline_layout: WebGPUPipelineLayout,
    valid: Cell<bool>,
}

impl GPUPipelineLayout {
    fn new_inherited(pipeline_layout: WebGPUPipelineLayout, valid: bool) -> Self {
        Self {
            reflector_: Reflector::new(),
            label: DomRefCell::new(None),
            pipeline_layout,
            valid: Cell::new(valid),
        }
    }

    pub fn new(
        global: &GlobalScope,
        pipeline_layout: WebGPUPipelineLayout,
        valid: bool,
    ) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(GPUPipelineLayout::new_inherited(pipeline_layout, valid)),
            global,
        )
    }
}

impl GPUPipelineLayout {
    pub fn id(&self) -> WebGPUPipelineLayout {
        self.pipeline_layout
    }

    pub fn is_valid(&self) -> bool {
        self.valid.get()
    }
}

impl GPUPipelineLayoutMethods for GPUPipelineLayout {
    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn GetLabel(&self) -> Option<DOMString> {
        self.label.borrow().clone()
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label
    fn SetLabel(&self, value: Option<DOMString>) {
        *self.label.borrow_mut() = value;
    }
}
