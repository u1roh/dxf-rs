// other implementation is in `generated/entities.rs`

use enum_primitive::FromPrimitive;
use std::io::{Read, Write};

use crate::{CodePair, Color, DxfError, DxfResult, Handle, Point, Vector};

use crate::code_pair_put_back::CodePairPutBack;
use crate::code_pair_writer::CodePairWriter;
use crate::entities::*;
use crate::enums::*;
use crate::helper_functions::*;
use crate::Drawing;

//------------------------------------------------------------------------------
//                                                                           Arc
//------------------------------------------------------------------------------
impl Arc {
    pub fn new(center: Point, radius: f64, start: f64, end: f64) -> Self {
        Arc {
            center,
            radius,
            start_angle: start,
            end_angle: end,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                        Circle
//------------------------------------------------------------------------------
impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Circle {
            center,
            radius,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                 DimensionBase
//------------------------------------------------------------------------------
impl DimensionBase {
    fn set_dimension_type(&mut self, val: i16) -> DxfResult<()> {
        self.is_block_reference_referenced_by_this_block_only = (val & 32) == 32;
        self.is_ordinate_x_type = (val & 64) == 64;
        self.is_at_user_defined_location = (val & 128) == 128;
        self.dimension_type = enum_from_number!(DimensionType, Aligned, from_i16, val & 0x0F); // only take the lower 4 bits
        Ok(())
    }
    pub(crate) fn get_dimension_type(&self) -> i16 {
        let mut val = self.dimension_type as i16;
        if self.is_block_reference_referenced_by_this_block_only {
            val |= 32;
        }
        if self.is_ordinate_x_type {
            val |= 64;
        }
        if self.is_at_user_defined_location {
            val |= 128;
        }
        val
    }
    fn apply_dimension_code_pair(&mut self, pair: &CodePair) -> DxfResult<bool> {
        match pair.code {
            1 => {
                self.text = pair.assert_string()?;
            }
            2 => {
                self.block_name = pair.assert_string()?;
            }
            3 => {
                self.dimension_style_name = pair.assert_string()?;
            }
            10 => {
                self.definition_point_1.x = pair.assert_f64()?;
            }
            20 => {
                self.definition_point_1.y = pair.assert_f64()?;
            }
            30 => {
                self.definition_point_1.z = pair.assert_f64()?;
            }
            11 => {
                self.text_mid_point.x = pair.assert_f64()?;
            }
            21 => {
                self.text_mid_point.y = pair.assert_f64()?;
            }
            31 => {
                self.text_mid_point.z = pair.assert_f64()?;
            }
            41 => {
                self.text_line_spacing_factor = pair.assert_f64()?;
            }
            42 => {
                self.actual_measurement = pair.assert_f64()?;
            }
            51 => {
                self.horizontal_direction_angle = pair.assert_f64()?;
            }
            53 => {
                self.text_rotation_angle = pair.assert_f64()?;
            }
            70 => {
                self.set_dimension_type(pair.assert_i16()?)?;
            }
            71 => {
                self.attachment_point =
                    enum_from_number!(AttachmentPoint, TopLeft, from_i16, pair.assert_i16()?);
            }
            72 => {
                self.text_line_spacing_style =
                    enum_from_number!(TextLineSpacingStyle, AtLeast, from_i16, pair.assert_i16()?);
            }
            210 => {
                self.normal.x = pair.assert_f64()?;
            }
            220 => {
                self.normal.y = pair.assert_f64()?;
            }
            230 => {
                self.normal.z = pair.assert_f64()?;
            }
            280 => {
                self.version = enum_from_number!(Version, R2010, from_i16, pair.assert_i16()?);
            }
            _ => return Ok(false),
        }
        Ok(true)
    }
}

//------------------------------------------------------------------------------
//                                                                        Face3D
//------------------------------------------------------------------------------
impl Face3D {
    pub fn new(
        first_corner: Point,
        second_corner: Point,
        third_corner: Point,
        fourth_corner: Point,
    ) -> Self {
        Face3D {
            first_corner,
            second_corner,
            third_corner,
            fourth_corner,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                        Insert
//------------------------------------------------------------------------------
impl Insert {
    pub fn attributes(&self) -> impl Iterator<Item = &Attribute> {
        self.__attributes_and_handles.iter().map(|a| &a.0)
    }
    pub fn attributes_mut(&mut self) -> impl Iterator<Item = &mut Attribute> {
        self.__attributes_and_handles.iter_mut().map(|a| &mut a.0)
    }
    pub fn add_attribute(&mut self, drawing: &mut Drawing, att: Attribute) {
        let att_handle = drawing.next_handle();
        self.__attributes_and_handles.push((att, att_handle));
    }
}

//------------------------------------------------------------------------------
//                                                                          Line
//------------------------------------------------------------------------------
impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Line {
            p1,
            p2,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                              LwPolylineVertex
//------------------------------------------------------------------------------
/// Represents a single vertex of a `LwPolyline`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct LwPolylineVertex {
    pub x: f64,
    pub y: f64,
    pub id: i32,
    pub starting_width: f64,
    pub ending_width: f64,
    pub bulge: f64,
}

//------------------------------------------------------------------------------
//                                                                    ModelPoint
//------------------------------------------------------------------------------
impl ModelPoint {
    pub fn new(p: Point) -> Self {
        ModelPoint {
            location: p,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                      Polyline
//------------------------------------------------------------------------------
impl Polyline {
    pub fn vertices(&self) -> impl Iterator<Item = &Vertex> {
        self.__vertices_and_handles.iter().map(|v| &v.0)
    }
    pub fn vertices_mut(&mut self) -> impl Iterator<Item = &mut Vertex> {
        self.__vertices_and_handles.iter_mut().map(|v| &mut v.0)
    }
    pub fn add_vertex(&mut self, drawing: &mut Drawing, vertex: Vertex) {
        let vertex_handle = drawing.next_handle();
        self.__vertices_and_handles.push((vertex, vertex_handle));
    }
}

//------------------------------------------------------------------------------
//                                                                   ProxyEntity
//------------------------------------------------------------------------------
impl ProxyEntity {
    // lower word
    pub fn get_object_drawing_format_version(&self) -> i32 {
        (self.__object_drawing_format & 0xFFFF) as i32
    }
    pub fn set_object_drawing_format_version(&mut self, version: i32) {
        self.__object_drawing_format |= version as u32 & 0xFFFF;
    }
    // upper word
    pub fn get_object_maintenance_release_version(&self) -> i32 {
        self.__object_drawing_format as i32 >> 4
    }
    pub fn set_object_mainenance_release_version(&mut self, version: i32) {
        self.__object_drawing_format =
            (version << 4) as u32 + (self.__object_drawing_format & 0xFFFF);
    }
}

//------------------------------------------------------------------------------
//                                                                         Solid
//------------------------------------------------------------------------------
impl Solid {
    pub fn new(
        first_corner: Point,
        second_corner: Point,
        third_corner: Point,
        fourth_corner: Point,
    ) -> Self {
        Solid {
            first_corner,
            second_corner,
            third_corner,
            fourth_corner,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                         Trace
//------------------------------------------------------------------------------
impl Trace {
    pub fn new(
        first_corner: Point,
        second_corner: Point,
        third_corner: Point,
        fourth_corner: Point,
    ) -> Self {
        Trace {
            first_corner,
            second_corner,
            third_corner,
            fourth_corner,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                        Vertex
//------------------------------------------------------------------------------
impl Vertex {
    pub fn new(location: Point) -> Self {
        Vertex {
            location,
            ..Default::default()
        }
    }
}

//------------------------------------------------------------------------------
//                                                                    EntityType
//------------------------------------------------------------------------------
impl EntityType {
    fn apply_dimension_code_pair(&mut self, pair: &CodePair) -> DxfResult<bool> {
        match *self {
            EntityType::RotatedDimension(ref mut dim) => match pair.code {
                12 => {
                    dim.insertion_point.x = pair.assert_f64()?;
                }
                22 => {
                    dim.insertion_point.y = pair.assert_f64()?;
                }
                32 => {
                    dim.insertion_point.z = pair.assert_f64()?;
                }
                13 => {
                    dim.definition_point_2.x = pair.assert_f64()?;
                }
                23 => {
                    dim.definition_point_2.y = pair.assert_f64()?;
                }
                33 => {
                    dim.definition_point_2.z = pair.assert_f64()?;
                }
                14 => {
                    dim.definition_point_3.x = pair.assert_f64()?;
                }
                24 => {
                    dim.definition_point_3.y = pair.assert_f64()?;
                }
                34 => {
                    dim.definition_point_3.z = pair.assert_f64()?;
                }
                50 => {
                    dim.rotation_angle = pair.assert_f64()?;
                }
                52 => {
                    dim.extension_line_angle = pair.assert_f64()?;
                }
                _ => {
                    return dim.dimension_base.apply_dimension_code_pair(pair);
                }
            },
            EntityType::RadialDimension(ref mut dim) => match pair.code {
                15 => {
                    dim.definition_point_2.x = pair.assert_f64()?;
                }
                25 => {
                    dim.definition_point_2.y = pair.assert_f64()?;
                }
                35 => {
                    dim.definition_point_2.z = pair.assert_f64()?;
                }
                40 => {
                    dim.leader_length = pair.assert_f64()?;
                }
                _ => {
                    return dim.dimension_base.apply_dimension_code_pair(pair);
                }
            },
            EntityType::DiameterDimension(ref mut dim) => match pair.code {
                15 => {
                    dim.definition_point_2.x = pair.assert_f64()?;
                }
                25 => {
                    dim.definition_point_2.y = pair.assert_f64()?;
                }
                35 => {
                    dim.definition_point_2.z = pair.assert_f64()?;
                }
                40 => {
                    dim.leader_length = pair.assert_f64()?;
                }
                _ => {
                    return dim.dimension_base.apply_dimension_code_pair(pair);
                }
            },
            EntityType::AngularThreePointDimension(ref mut dim) => match pair.code {
                13 => {
                    dim.definition_point_2.x = pair.assert_f64()?;
                }
                23 => {
                    dim.definition_point_2.y = pair.assert_f64()?;
                }
                33 => {
                    dim.definition_point_2.z = pair.assert_f64()?;
                }
                14 => {
                    dim.definition_point_3.x = pair.assert_f64()?;
                }
                24 => {
                    dim.definition_point_3.y = pair.assert_f64()?;
                }
                34 => {
                    dim.definition_point_3.z = pair.assert_f64()?;
                }
                15 => {
                    dim.definition_point_4.x = pair.assert_f64()?;
                }
                25 => {
                    dim.definition_point_4.y = pair.assert_f64()?;
                }
                35 => {
                    dim.definition_point_4.z = pair.assert_f64()?;
                }
                16 => {
                    dim.definition_point_5.x = pair.assert_f64()?;
                }
                26 => {
                    dim.definition_point_5.y = pair.assert_f64()?;
                }
                36 => {
                    dim.definition_point_5.z = pair.assert_f64()?;
                }
                _ => {
                    return dim.dimension_base.apply_dimension_code_pair(pair);
                }
            },
            EntityType::OrdinateDimension(ref mut dim) => match pair.code {
                13 => {
                    dim.definition_point_2.x = pair.assert_f64()?;
                }
                23 => {
                    dim.definition_point_2.y = pair.assert_f64()?;
                }
                33 => {
                    dim.definition_point_2.z = pair.assert_f64()?;
                }
                14 => {
                    dim.definition_point_3.x = pair.assert_f64()?;
                }
                24 => {
                    dim.definition_point_3.y = pair.assert_f64()?;
                }
                34 => {
                    dim.definition_point_3.z = pair.assert_f64()?;
                }
                _ => {
                    return dim.dimension_base.apply_dimension_code_pair(pair);
                }
            },
            _ => {
                return Err(DxfError::UnexpectedEnumValue(pair.offset));
            }
        }
        Ok(true)
    }
}

//------------------------------------------------------------------------------
//                                                                  EntityCommon
//------------------------------------------------------------------------------
impl EntityCommon {
    /// Ensures all values are valid.
    pub fn normalize(&mut self) {
        default_if_empty(&mut self.layer, "0");
    }
}

//------------------------------------------------------------------------------
//                                                                        Entity
//------------------------------------------------------------------------------
impl Entity {
    /// Creates a new `Entity` with the default common values.
    pub fn new(specific: EntityType) -> Self {
        Entity {
            common: Default::default(),
            specific,
        }
    }
    /// Ensures all entity values are valid.
    pub fn normalize(&mut self) {
        self.common.normalize();
        // no entity-specific values to set
    }
    pub(crate) fn read<I>(iter: &mut CodePairPutBack<I>) -> DxfResult<Option<Entity>>
    where
        I: Read,
    {
        loop {
            match iter.next() {
                // first code pair must be 0/entity-type
                Some(Ok(pair @ CodePair { code: 0, .. })) => {
                    let type_string = pair.assert_string()?;
                    if type_string == "ENDSEC" || type_string == "ENDBLK" {
                        iter.put_back(Ok(pair));
                        return Ok(None);
                    }

                    match &*type_string {
                        "DIMENSION" => {
                            // dimensions require special handling
                            let mut common = EntityCommon::default();
                            let mut dim = EntityType::RotatedDimension(Default::default());
                            loop {
                                match iter.next() {
                                    Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                        // new entity or ENDSEC
                                        iter.put_back(Ok(pair));
                                        break;
                                    }
                                    Some(Ok(pair)) => {
                                        if pair.code == 100 {
                                            let base = match &dim {
                                                EntityType::RotatedDimension(dim) => {
                                                    &dim.dimension_base
                                                }
                                                EntityType::RadialDimension(dim) => {
                                                    &dim.dimension_base
                                                }
                                                EntityType::DiameterDimension(dim) => {
                                                    &dim.dimension_base
                                                }
                                                EntityType::AngularThreePointDimension(dim) => {
                                                    &dim.dimension_base
                                                }
                                                EntityType::OrdinateDimension(dim) => {
                                                    &dim.dimension_base
                                                }
                                                _ => unreachable!(),
                                            };
                                            match &*pair.assert_string()? {
                                                "AcDbRadialDimension" => {
                                                    dim = EntityType::RadialDimension(
                                                        RadialDimension {
                                                            dimension_base: base.clone(),
                                                            ..Default::default()
                                                        },
                                                    );
                                                }
                                                "AcDbDiametricDimension" => {
                                                    dim = EntityType::DiameterDimension(
                                                        DiameterDimension {
                                                            dimension_base: base.clone(),
                                                            ..Default::default()
                                                        },
                                                    );
                                                }
                                                "AcDb3PointAngularDimension" => {
                                                    dim = EntityType::AngularThreePointDimension(
                                                        AngularThreePointDimension {
                                                            dimension_base: base.clone(),
                                                            ..Default::default()
                                                        },
                                                    );
                                                }
                                                "AcDbOrdinateDimension" => {
                                                    dim = EntityType::OrdinateDimension(
                                                        OrdinateDimension {
                                                            dimension_base: base.clone(),
                                                            ..Default::default()
                                                        },
                                                    );
                                                }
                                                _ => {} // unexpected dimension type
                                            }
                                        } else if !dim.apply_dimension_code_pair(&pair)? {
                                            common.apply_individual_pair(&pair, iter)?;
                                        }
                                    }
                                    Some(Err(e)) => return Err(e),
                                    None => return Err(DxfError::UnexpectedEndOfInput),
                                }
                            }
                            return Ok(Some(Entity {
                                common,
                                specific: dim,
                            }));
                        }
                        _ => {
                            match EntityType::from_type_string(&type_string) {
                                Some(e) => {
                                    let mut entity = Entity::new(e);
                                    if !entity.apply_custom_reader(iter)? {
                                        // no custom reader, use the auto-generated one
                                        loop {
                                            match iter.next() {
                                                Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                                    // new entity or ENDSEC
                                                    iter.put_back(Ok(pair));
                                                    break;
                                                }
                                                Some(Ok(pair)) => {
                                                    entity.apply_code_pair(&pair, iter)?
                                                }
                                                Some(Err(e)) => return Err(e),
                                                None => return Err(DxfError::UnexpectedEndOfInput),
                                            }
                                        }

                                        entity.post_parse()?;
                                    }

                                    return Ok(Some(entity));
                                }
                                None => {
                                    // swallow unsupported entity
                                    loop {
                                        match iter.next() {
                                            Some(Ok(pair @ CodePair { code: 0, .. })) => {
                                                // found another entity or ENDSEC
                                                iter.put_back(Ok(pair));
                                                break;
                                            }
                                            Some(Ok(_)) => (), // part of the unsupported entity
                                            Some(Err(e)) => return Err(e),
                                            None => return Err(DxfError::UnexpectedEndOfInput),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Ok(pair)) => {
                    return Err(DxfError::UnexpectedCodePair(
                        pair,
                        String::from("expected 0/entity-type or 0/ENDSEC"),
                    ))
                }
                Some(Err(e)) => return Err(e),
                None => return Err(DxfError::UnexpectedEndOfInput),
            }
        }
    }
    fn apply_code_pair<I>(
        &mut self,
        pair: &CodePair,
        iter: &mut CodePairPutBack<I>,
    ) -> DxfResult<()>
    where
        I: Read,
    {
        if !self.specific.try_apply_code_pair(&pair)? {
            self.common.apply_individual_pair(&pair, iter)?;
        }
        Ok(())
    }
    fn post_parse(&mut self) -> DxfResult<()> {
        match self.specific {
            EntityType::Image(ref mut image) => {
                combine_points_2(
                    &mut image.__clipping_vertices_x,
                    &mut image.__clipping_vertices_y,
                    &mut image.clipping_vertices,
                    Point::new,
                );
            }
            EntityType::Leader(ref mut leader) => {
                combine_points_3(
                    &mut leader.__vertices_x,
                    &mut leader.__vertices_y,
                    &mut leader.__vertices_z,
                    &mut leader.vertices,
                    Point::new,
                );
            }
            EntityType::MLine(ref mut mline) => {
                combine_points_3(
                    &mut mline.__vertices_x,
                    &mut mline.__vertices_y,
                    &mut mline.__vertices_z,
                    &mut mline.vertices,
                    Point::new,
                );
                combine_points_3(
                    &mut mline.__segment_direction_x,
                    &mut mline.__segment_direction_y,
                    &mut mline.__segment_direction_z,
                    &mut mline.segment_directions,
                    Vector::new,
                );
                combine_points_3(
                    &mut mline.__miter_direction_x,
                    &mut mline.__miter_direction_y,
                    &mut mline.__miter_direction_z,
                    &mut mline.miter_directions,
                    Vector::new,
                );
            }
            EntityType::Section(ref mut section) => {
                combine_points_3(
                    &mut section.__vertices_x,
                    &mut section.__vertices_y,
                    &mut section.__vertices_z,
                    &mut section.vertices,
                    Point::new,
                );
                combine_points_3(
                    &mut section.__back_line_vertices_x,
                    &mut section.__back_line_vertices_y,
                    &mut section.__back_line_vertices_z,
                    &mut section.back_line_vertices,
                    Point::new,
                );
            }
            EntityType::Spline(ref mut spline) => {
                combine_points_3(
                    &mut spline.__control_point_x,
                    &mut spline.__control_point_y,
                    &mut spline.__control_point_z,
                    &mut spline.control_points,
                    Point::new,
                );
                combine_points_3(
                    &mut spline.__fit_point_x,
                    &mut spline.__fit_point_y,
                    &mut spline.__fit_point_z,
                    &mut spline.fit_points,
                    Point::new,
                );
            }
            EntityType::DgnUnderlay(ref mut underlay) => {
                combine_points_2(
                    &mut underlay.__point_x,
                    &mut underlay.__point_y,
                    &mut underlay.points,
                    Point::new,
                );
            }
            EntityType::DwfUnderlay(ref mut underlay) => {
                combine_points_2(
                    &mut underlay.__point_x,
                    &mut underlay.__point_y,
                    &mut underlay.points,
                    Point::new,
                );
            }
            EntityType::PdfUnderlay(ref mut underlay) => {
                combine_points_2(
                    &mut underlay.__point_x,
                    &mut underlay.__point_y,
                    &mut underlay.points,
                    Point::new,
                );
            }
            EntityType::Wipeout(ref mut wo) => {
                combine_points_2(
                    &mut wo.__clipping_vertices_x,
                    &mut wo.__clipping_vertices_y,
                    &mut wo.clipping_vertices,
                    Point::new,
                );
            }
            _ => (),
        }

        Ok(())
    }
    fn apply_custom_reader<I>(&mut self, iter: &mut CodePairPutBack<I>) -> DxfResult<bool>
    where
        I: Read,
    {
        match self.specific {
            EntityType::Attribute(ref mut att) => {
                Entity::apply_custom_reader_attribute(&mut self.common, att, iter)
            }
            EntityType::AttributeDefinition(ref mut att) => {
                Entity::apply_custom_reader_attributedefinition(&mut self.common, att, iter)
            }
            EntityType::LwPolyline(ref mut poly) => {
                Entity::apply_custom_reader_lwpolyline(&mut self.common, poly, iter)
            }
            EntityType::MText(ref mut mtext) => {
                Entity::apply_custom_reader_mtext(&mut self.common, mtext, iter)
            }
            _ => Ok(false), // no custom reader
        }
    }
    fn apply_custom_reader_attribute<I>(
        common: &mut EntityCommon,
        att: &mut Attribute,
        iter: &mut CodePairPutBack<I>,
    ) -> DxfResult<bool>
    where
        I: Read,
    {
        let xrecord_text = "AcDbXrecord";
        let mut last_subclass_marker = String::new();
        let mut is_version_set = false;
        let mut xrec_code_70_count = 0;
        loop {
            let pair = next_pair!(iter);
            match pair.code {
                100 => {
                    last_subclass_marker = pair.assert_string()?;
                }
                1 => {
                    att.value = pair.assert_string()?;
                }
                2 => {
                    if last_subclass_marker == xrecord_text {
                        att.x_record_tag = pair.assert_string()?;
                    } else {
                        att.attribute_tag = pair.assert_string()?;
                    }
                }
                7 => {
                    att.text_style_name = pair.assert_string()?;
                }
                10 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.x = pair.assert_f64()?;
                    } else {
                        att.location.x = pair.assert_f64()?;
                    }
                }
                20 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.y = pair.assert_f64()?;
                    } else {
                        att.location.y = pair.assert_f64()?;
                    }
                }
                30 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.z = pair.assert_f64()?;
                    } else {
                        att.location.z = pair.assert_f64()?;
                    }
                }
                11 => {
                    att.second_alignment_point.x = pair.assert_f64()?;
                }
                21 => {
                    att.second_alignment_point.y = pair.assert_f64()?;
                }
                31 => {
                    att.second_alignment_point.z = pair.assert_f64()?;
                }
                39 => {
                    att.thickness = pair.assert_f64()?;
                }
                40 => {
                    if last_subclass_marker == xrecord_text {
                        att.annotation_scale = pair.assert_f64()?;
                    } else {
                        att.text_height = pair.assert_f64()?;
                    }
                }
                41 => {
                    att.relative_x_scale_factor = pair.assert_f64()?;
                }
                50 => {
                    att.rotation = pair.assert_f64()?;
                }
                51 => {
                    att.oblique_angle = pair.assert_f64()?;
                }
                70 => {
                    if last_subclass_marker == xrecord_text {
                        match xrec_code_70_count {
                            0 => {
                                att.m_text_flag = enum_from_number!(
                                    MTextFlag,
                                    MultilineAttribute,
                                    from_i16,
                                    pair.assert_i16()?
                                )
                            }
                            1 => att.is_really_locked = as_bool(pair.assert_i16()?),
                            2 => att.__secondary_attribute_count = i32::from(pair.assert_i16()?),
                            _ => return Err(DxfError::UnexpectedCodePair(pair, String::new())),
                        }
                        xrec_code_70_count += 1;
                    } else {
                        att.flags = i32::from(pair.assert_i16()?);
                    }
                }
                71 => {
                    att.text_generation_flags = i32::from(pair.assert_i16()?);
                }
                72 => {
                    att.horizontal_text_justification = enum_from_number!(
                        HorizontalTextJustification,
                        Left,
                        from_i16,
                        pair.assert_i16()?
                    );
                }
                73 => {
                    att.field_length = pair.assert_i16()?;
                }
                74 => {
                    att.vertical_text_justification = enum_from_number!(
                        VerticalTextJustification,
                        Baseline,
                        from_i16,
                        pair.assert_i16()?
                    );
                }
                210 => {
                    att.normal.x = pair.assert_f64()?;
                }
                220 => {
                    att.normal.y = pair.assert_f64()?;
                }
                230 => {
                    att.normal.z = pair.assert_f64()?;
                }
                280 => {
                    if last_subclass_marker == xrecord_text {
                        att.keep_duplicate_records = as_bool(pair.assert_i16()?);
                    } else if !is_version_set {
                        att.version =
                            enum_from_number!(Version, R2010, from_i16, pair.assert_i16()?);
                        is_version_set = true;
                    } else {
                        att.is_locked_in_block = as_bool(pair.assert_i16()?);
                    }
                }
                340 => {
                    att.__secondary_attributes_handle.push(pair.as_handle()?);
                }
                _ => {
                    common.apply_individual_pair(&pair, iter)?;
                }
            }
        }
    }
    fn apply_custom_reader_attributedefinition<I>(
        common: &mut EntityCommon,
        att: &mut AttributeDefinition,
        iter: &mut CodePairPutBack<I>,
    ) -> DxfResult<bool>
    where
        I: Read,
    {
        let xrecord_text = "AcDbXrecord";
        let mut last_subclass_marker = String::new();
        let mut is_version_set = false;
        let mut xrec_code_70_count = 0;
        loop {
            let pair = next_pair!(iter);
            match pair.code {
                100 => {
                    last_subclass_marker = pair.assert_string()?;
                }
                1 => {
                    att.value = pair.assert_string()?;
                }
                2 => {
                    if last_subclass_marker == xrecord_text {
                        att.x_record_tag = pair.assert_string()?;
                    } else {
                        att.text_tag = pair.assert_string()?;
                    }
                }
                3 => {
                    att.prompt = pair.assert_string()?;
                }
                7 => {
                    att.text_style_name = pair.assert_string()?;
                }
                10 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.x = pair.assert_f64()?;
                    } else {
                        att.location.x = pair.assert_f64()?;
                    }
                }
                20 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.y = pair.assert_f64()?;
                    } else {
                        att.location.y = pair.assert_f64()?;
                    }
                }
                30 => {
                    if last_subclass_marker == xrecord_text {
                        att.alignment_point.z = pair.assert_f64()?;
                    } else {
                        att.location.z = pair.assert_f64()?;
                    }
                }
                11 => {
                    att.second_alignment_point.x = pair.assert_f64()?;
                }
                21 => {
                    att.second_alignment_point.y = pair.assert_f64()?;
                }
                31 => {
                    att.second_alignment_point.z = pair.assert_f64()?;
                }
                39 => {
                    att.thickness = pair.assert_f64()?;
                }
                40 => {
                    if last_subclass_marker == xrecord_text {
                        att.annotation_scale = pair.assert_f64()?;
                    } else {
                        att.text_height = pair.assert_f64()?;
                    }
                }
                41 => {
                    att.relative_x_scale_factor = pair.assert_f64()?;
                }
                50 => {
                    att.rotation = pair.assert_f64()?;
                }
                51 => {
                    att.oblique_angle = pair.assert_f64()?;
                }
                70 => {
                    if last_subclass_marker == xrecord_text {
                        match xrec_code_70_count {
                            0 => {
                                att.m_text_flag = enum_from_number!(
                                    MTextFlag,
                                    MultilineAttribute,
                                    from_i16,
                                    pair.assert_i16()?
                                )
                            }
                            1 => att.is_really_locked = as_bool(pair.assert_i16()?),
                            2 => att.__secondary_attribute_count = i32::from(pair.assert_i16()?),
                            _ => return Err(DxfError::UnexpectedCodePair(pair, String::new())),
                        }
                        xrec_code_70_count += 1;
                    } else {
                        att.flags = i32::from(pair.assert_i16()?);
                    }
                }
                71 => {
                    att.text_generation_flags = i32::from(pair.assert_i16()?);
                }
                72 => {
                    att.horizontal_text_justification = enum_from_number!(
                        HorizontalTextJustification,
                        Left,
                        from_i16,
                        pair.assert_i16()?
                    );
                }
                73 => {
                    att.field_length = pair.assert_i16()?;
                }
                74 => {
                    att.vertical_text_justification = enum_from_number!(
                        VerticalTextJustification,
                        Baseline,
                        from_i16,
                        pair.assert_i16()?
                    );
                }
                210 => {
                    att.normal.x = pair.assert_f64()?;
                }
                220 => {
                    att.normal.y = pair.assert_f64()?;
                }
                230 => {
                    att.normal.z = pair.assert_f64()?;
                }
                280 => {
                    if last_subclass_marker == xrecord_text {
                        att.keep_duplicate_records = as_bool(pair.assert_i16()?);
                    } else if !is_version_set {
                        att.version =
                            enum_from_number!(Version, R2010, from_i16, pair.assert_i16()?);
                        is_version_set = true;
                    } else {
                        att.is_locked_in_block = as_bool(pair.assert_i16()?);
                    }
                }
                340 => {
                    att.__secondary_attributes_handle.push(pair.as_handle()?);
                }
                _ => {
                    common.apply_individual_pair(&pair, iter)?;
                }
            }
        }
    }
    fn apply_custom_reader_lwpolyline<I>(
        common: &mut EntityCommon,
        poly: &mut LwPolyline,
        iter: &mut CodePairPutBack<I>,
    ) -> DxfResult<bool>
    where
        I: Read,
    {
        loop {
            let pair = next_pair!(iter);
            match pair.code {
                // vertex-specific pairs
                10 => {
                    // start a new vertex
                    poly.vertices.push(LwPolylineVertex::default());
                    vec_last!(poly.vertices).x = pair.assert_f64()?;
                }
                20 => {
                    vec_last!(poly.vertices).y = pair.assert_f64()?;
                }
                40 => {
                    vec_last!(poly.vertices).starting_width = pair.assert_f64()?;
                }
                41 => {
                    vec_last!(poly.vertices).ending_width = pair.assert_f64()?;
                }
                42 => {
                    vec_last!(poly.vertices).bulge = pair.assert_f64()?;
                }
                91 => {
                    vec_last!(poly.vertices).id = pair.assert_i32()?;
                }
                // other pairs
                39 => {
                    poly.thickness = pair.assert_f64()?;
                }
                43 => {
                    poly.constant_width = pair.assert_f64()?;
                }
                70 => {
                    poly.flags = i32::from(pair.assert_i16()?);
                }
                210 => {
                    poly.extrusion_direction.x = pair.assert_f64()?;
                }
                220 => {
                    poly.extrusion_direction.y = pair.assert_f64()?;
                }
                230 => {
                    poly.extrusion_direction.z = pair.assert_f64()?;
                }
                _ => {
                    common.apply_individual_pair(&pair, iter)?;
                }
            }
        }
    }
    fn apply_custom_reader_mtext<I>(
        common: &mut EntityCommon,
        mtext: &mut MText,
        iter: &mut CodePairPutBack<I>,
    ) -> DxfResult<bool>
    where
        I: Read,
    {
        let mut reading_column_data = false;
        let mut read_column_count = false;
        loop {
            let pair = next_pair!(iter);
            match pair.code {
                10 => {
                    mtext.insertion_point.x = pair.assert_f64()?;
                }
                20 => {
                    mtext.insertion_point.y = pair.assert_f64()?;
                }
                30 => {
                    mtext.insertion_point.z = pair.assert_f64()?;
                }
                40 => {
                    mtext.initial_text_height = pair.assert_f64()?;
                }
                41 => {
                    mtext.reference_rectangle_width = pair.assert_f64()?;
                }
                71 => {
                    mtext.attachment_point =
                        enum_from_number!(AttachmentPoint, TopLeft, from_i16, pair.assert_i16()?);
                }
                72 => {
                    mtext.drawing_direction = enum_from_number!(
                        DrawingDirection,
                        LeftToRight,
                        from_i16,
                        pair.assert_i16()?
                    );
                }
                3 => {
                    mtext.extended_text.push(pair.assert_string()?);
                }
                1 => {
                    mtext.text = pair.assert_string()?;
                }
                7 => {
                    mtext.text_style_name = pair.assert_string()?;
                }
                210 => {
                    mtext.extrusion_direction.x = pair.assert_f64()?;
                }
                220 => {
                    mtext.extrusion_direction.y = pair.assert_f64()?;
                }
                230 => {
                    mtext.extrusion_direction.z = pair.assert_f64()?;
                }
                11 => {
                    mtext.x_axis_direction.x = pair.assert_f64()?;
                }
                21 => {
                    mtext.x_axis_direction.y = pair.assert_f64()?;
                }
                31 => {
                    mtext.x_axis_direction.z = pair.assert_f64()?;
                }
                42 => {
                    mtext.horizontal_width = pair.assert_f64()?;
                }
                43 => {
                    mtext.vertical_height = pair.assert_f64()?;
                }
                50 => {
                    if reading_column_data {
                        if read_column_count {
                            mtext.column_heights.push(pair.assert_f64()?);
                        } else {
                            mtext.column_count = pair.assert_f64()? as i32;
                            read_column_count = true;
                        }
                    } else {
                        mtext.rotation_angle = pair.assert_f64()?;
                    }
                }
                73 => {
                    mtext.line_spacing_style = enum_from_number!(
                        MTextLineSpacingStyle,
                        AtLeast,
                        from_i16,
                        pair.assert_i16()?
                    );
                }
                44 => {
                    mtext.line_spacing_factor = pair.assert_f64()?;
                }
                90 => {
                    mtext.background_fill_setting =
                        enum_from_number!(BackgroundFillSetting, Off, from_i32, pair.assert_i32()?);
                }
                420 => {
                    mtext.background_color_rgb = pair.assert_i32()?;
                }
                430 => {
                    mtext.background_color_name = pair.assert_string()?;
                }
                45 => {
                    mtext.fill_box_scale = pair.assert_f64()?;
                }
                63 => {
                    mtext.background_fill_color = Color::from_raw_value(pair.assert_i16()?);
                }
                441 => {
                    mtext.background_fill_color_transparency = pair.assert_i32()?;
                }
                75 => {
                    mtext.column_type = pair.assert_i16()?;
                    reading_column_data = true;
                }
                76 => {
                    mtext.column_count = i32::from(pair.assert_i16()?);
                }
                78 => {
                    mtext.is_column_flow_reversed = as_bool(pair.assert_i16()?);
                }
                79 => {
                    mtext.is_column_auto_height = as_bool(pair.assert_i16()?);
                }
                48 => {
                    mtext.column_width = pair.assert_f64()?;
                }
                49 => {
                    mtext.column_gutter = pair.assert_f64()?;
                }
                _ => {
                    common.apply_individual_pair(&pair, iter)?;
                }
            }
        }
    }
    pub(crate) fn write<T>(
        &self,
        version: AcadVersion,
        write_handles: bool,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<()>
    where
        T: Write + ?Sized,
    {
        if self.specific.is_supported_on_version(version) {
            writer.write_code_pair(&CodePair::new_str(0, self.specific.to_type_string()))?;
            self.common.write(version, write_handles, writer)?;
            if !self.apply_custom_writer(version, writer)? {
                self.specific.write(&self.common, version, writer)?;
            }

            self.post_write(version, write_handles, writer)?;
            for x in &self.common.x_data {
                x.write(version, writer)?;
            }
        }

        Ok(())
    }
    fn apply_custom_writer<T>(
        &self,
        version: AcadVersion,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<bool>
    where
        T: Write + ?Sized,
    {
        match self.specific {
            EntityType::RotatedDimension(ref dim) => {
                Entity::apply_custom_writer_rotateddimension(dim, version, writer)?;
            }
            EntityType::RadialDimension(ref dim) => {
                Entity::apply_custom_writer_radialdimension(dim, version, writer)?;
            }
            EntityType::DiameterDimension(ref dim) => {
                Entity::apply_custom_writer_diameterdimension(dim, version, writer)?;
            }
            EntityType::AngularThreePointDimension(ref dim) => {
                Entity::apply_custom_writer_angularthreepointdimension(dim, version, writer)?;
            }
            EntityType::OrdinateDimension(ref dim) => {
                Entity::apply_custom_writer_ordinatedimension(dim, version, writer)?;
            }
            EntityType::Polyline(ref poly) => {
                Entity::apply_custom_writer_polyline(poly, version, writer)?;
            }
            EntityType::Vertex(ref v) => {
                Entity::apply_custom_writer_vertex(v, version, writer)?;
            }
            _ => return Ok(false), // no custom writer
        }

        Ok(true)
    }
    fn apply_custom_writer_rotateddimension<T>(
        dim: &RotatedDimension,
        version: AcadVersion,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<bool>
    where
        T: Write + ?Sized,
    {
        dim.dimension_base.write(version, writer)?;
        if version >= AcadVersion::R13 {
            writer.write_code_pair(&CodePair::new_str(100, "AcDbAlignedDimension"))?;
        }
        writer.write_code_pair(&CodePair::new_f64(12, dim.insertion_point.x))?;
        writer.write_code_pair(&CodePair::new_f64(22, dim.insertion_point.y))?;
        writer.write_code_pair(&CodePair::new_f64(32, dim.insertion_point.z))?;
        writer.write_code_pair(&CodePair::new_f64(13, dim.definition_point_2.x))?;
        writer.write_code_pair(&CodePair::new_f64(23, dim.definition_point_2.y))?;
        writer.write_code_pair(&CodePair::new_f64(33, dim.definition_point_2.z))?;
        writer.write_code_pair(&CodePair::new_f64(14, dim.definition_point_3.x))?;
        writer.write_code_pair(&CodePair::new_f64(24, dim.definition_point_3.y))?;
        writer.write_code_pair(&CodePair::new_f64(34, dim.definition_point_3.z))?;
        writer.write_code_pair(&CodePair::new_f64(50, dim.rotation_angle))?;
        writer.write_code_pair(&CodePair::new_f64(52, dim.extension_line_angle))?;
        if version >= AcadVersion::R13 {
            writer.write_code_pair(&CodePair::new_str(100, "AcDbRotatedDimension"))?;
        }
        Ok(true)
    }
    fn apply_custom_writer_radialdimension<T>(
        dim: &RadialDimension,
        version: AcadVersion,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<bool>
    where
        T: Write + ?Sized,
    {
        dim.dimension_base.write(version, writer)?;
        writer.write_code_pair(&CodePair::new_str(100, "AcDbRadialDimension"))?;
        writer.write_code_pair(&CodePair::new_f64(15, dim.definition_point_2.x))?;
        writer.write_code_pair(&CodePair::new_f64(25, dim.definition_point_2.y))?;
        writer.write_code_pair(&CodePair::new_f64(35, dim.definition_point_2.z))?;
        writer.write_code_pair(&CodePair::new_f64(40, dim.leader_length))?;
        Ok(true)
    }
    fn apply_custom_writer_diameterdimension<T>(
        dim: &DiameterDimension,
        version: AcadVersion,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<bool>
    where
        T: Write + ?Sized,
    {
        dim.dimension_base.write(version, writer)?;
        writer.write_code_pair(&CodePair::new_str(100, "AcDbDiametricDimension"))?;
        writer.write_code_pair(&CodePair::new_f64(15, dim.definition_point_2.x))?;
        writer.write_code_pair(&CodePair::new_f64(25, dim.definition_point_2.y))?;
        writer.write_code_pair(&CodePair::new_f64(35, dim.definition_point_2.z))?;
        writer.write_code_pair(&CodePair::new_f64(40, dim.leader_length))?;
        Ok(true)
    }
    fn apply_custom_writer_angularthreepointdimension<T>(
        dim: &AngularThreePointDimension,
        version: AcadVersion,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<bool>
    where
        T: Write + ?Sized,
    {
        dim.dimension_base.write(version, writer)?;
        writer.write_code_pair(&CodePair::new_str(100, "AcDb3PointAngularDimension"))?;
        writer.write_code_pair(&CodePair::new_f64(13, dim.definition_point_2.x))?;
        writer.write_code_pair(&CodePair::new_f64(23, dim.definition_point_2.y))?;
        writer.write_code_pair(&CodePair::new_f64(33, dim.definition_point_2.z))?;
        writer.write_code_pair(&CodePair::new_f64(14, dim.definition_point_3.x))?;
        writer.write_code_pair(&CodePair::new_f64(24, dim.definition_point_3.y))?;
        writer.write_code_pair(&CodePair::new_f64(34, dim.definition_point_3.z))?;
        writer.write_code_pair(&CodePair::new_f64(15, dim.definition_point_4.x))?;
        writer.write_code_pair(&CodePair::new_f64(25, dim.definition_point_4.y))?;
        writer.write_code_pair(&CodePair::new_f64(35, dim.definition_point_4.z))?;
        writer.write_code_pair(&CodePair::new_f64(16, dim.definition_point_5.x))?;
        writer.write_code_pair(&CodePair::new_f64(26, dim.definition_point_5.y))?;
        writer.write_code_pair(&CodePair::new_f64(36, dim.definition_point_5.z))?;
        Ok(true)
    }
    fn apply_custom_writer_ordinatedimension<T>(
        dim: &OrdinateDimension,
        version: AcadVersion,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<bool>
    where
        T: Write + ?Sized,
    {
        dim.dimension_base.write(version, writer)?;
        writer.write_code_pair(&CodePair::new_str(100, "AcDbOrdinateDimension"))?;
        writer.write_code_pair(&CodePair::new_f64(13, dim.definition_point_2.x))?;
        writer.write_code_pair(&CodePair::new_f64(23, dim.definition_point_2.y))?;
        writer.write_code_pair(&CodePair::new_f64(33, dim.definition_point_2.z))?;
        writer.write_code_pair(&CodePair::new_f64(14, dim.definition_point_3.x))?;
        writer.write_code_pair(&CodePair::new_f64(24, dim.definition_point_3.y))?;
        writer.write_code_pair(&CodePair::new_f64(34, dim.definition_point_3.z))?;
        Ok(true)
    }
    fn apply_custom_writer_polyline<T>(
        poly: &Polyline,
        version: AcadVersion,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<bool>
    where
        T: Write + ?Sized,
    {
        let subclass_marker = if poly.get_is_3d_polyline() || poly.get_is_3d_polygon_mesh() {
            "AcDb3dPolyline"
        } else {
            "AcDb2dPolyline"
        };
        writer.write_code_pair(&CodePair::new_str(100, subclass_marker))?;
        if version <= AcadVersion::R13 {
            writer.write_code_pair(&CodePair::new_i16(66, as_i16(poly.contains_vertices)))?;
        }
        if version >= AcadVersion::R12 {
            writer.write_code_pair(&CodePair::new_f64(10, poly.location.x))?;
            writer.write_code_pair(&CodePair::new_f64(20, poly.location.y))?;
            writer.write_code_pair(&CodePair::new_f64(30, poly.location.z))?;
        }
        if poly.thickness != 0.0 {
            writer.write_code_pair(&CodePair::new_f64(39, poly.thickness))?;
        }
        if poly.flags != 0 {
            writer.write_code_pair(&CodePair::new_i16(70, poly.flags as i16))?;
        }
        if poly.default_starting_width != 0.0 {
            writer.write_code_pair(&CodePair::new_f64(40, poly.default_starting_width))?;
        }
        if poly.default_ending_width != 0.0 {
            writer.write_code_pair(&CodePair::new_f64(41, poly.default_ending_width))?;
        }
        if poly.polygon_mesh_m_vertex_count != 0 {
            writer.write_code_pair(&CodePair::new_i16(
                71,
                poly.polygon_mesh_m_vertex_count as i16,
            ))?;
        }
        if poly.polygon_mesh_n_vertex_count != 0 {
            writer.write_code_pair(&CodePair::new_i16(
                72,
                poly.polygon_mesh_n_vertex_count as i16,
            ))?;
        }
        if poly.smooth_surface_m_density != 0 {
            writer.write_code_pair(&CodePair::new_i16(73, poly.smooth_surface_m_density as i16))?;
        }
        if poly.smooth_surface_n_density != 0 {
            writer.write_code_pair(&CodePair::new_i16(74, poly.smooth_surface_n_density as i16))?;
        }
        if poly.surface_type != PolylineCurvedAndSmoothSurfaceType::None {
            writer.write_code_pair(&CodePair::new_i16(75, poly.surface_type as i16))?;
        }
        if poly.normal != Vector::z_axis() {
            writer.write_code_pair(&CodePair::new_f64(210, poly.normal.x))?;
            writer.write_code_pair(&CodePair::new_f64(220, poly.normal.y))?;
            writer.write_code_pair(&CodePair::new_f64(230, poly.normal.z))?;
        }
        Ok(true)
    }
    fn apply_custom_writer_vertex<T>(
        v: &Vertex,
        version: AcadVersion,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<bool>
    where
        T: Write + ?Sized,
    {
        writer.write_code_pair(&CodePair::new_str(100, "AcDbVertex"))?;
        let subclass_marker = if v.get_is_3d_polyline_vertex() || v.get_is_3d_polygon_mesh() {
            "AcDb3dPolylineVertex"
        } else {
            "AcDb2dVertex"
        };
        writer.write_code_pair(&CodePair::new_str(100, subclass_marker))?;
        writer.write_code_pair(&CodePair::new_f64(10, v.location.x))?;
        writer.write_code_pair(&CodePair::new_f64(20, v.location.y))?;
        writer.write_code_pair(&CodePair::new_f64(30, v.location.z))?;
        if v.starting_width != 0.0 {
            writer.write_code_pair(&CodePair::new_f64(40, v.starting_width))?;
        }
        if v.ending_width != 0.0 {
            writer.write_code_pair(&CodePair::new_f64(41, v.ending_width))?;
        }
        if v.bulge != 0.0 {
            writer.write_code_pair(&CodePair::new_f64(42, v.bulge))?;
        }
        writer.write_code_pair(&CodePair::new_i16(70, v.flags as i16))?;
        writer.write_code_pair(&CodePair::new_f64(50, v.curve_fit_tangent_direction))?;
        if version >= AcadVersion::R13 {
            if v.polyface_mesh_vertex_index1 != 0 {
                writer.write_code_pair(&CodePair::new_i16(
                    71,
                    v.polyface_mesh_vertex_index1 as i16,
                ))?;
            }
            if v.polyface_mesh_vertex_index2 != 0 {
                writer.write_code_pair(&CodePair::new_i16(
                    72,
                    v.polyface_mesh_vertex_index2 as i16,
                ))?;
            }
            if v.polyface_mesh_vertex_index3 != 0 {
                writer.write_code_pair(&CodePair::new_i16(
                    73,
                    v.polyface_mesh_vertex_index3 as i16,
                ))?;
            }
            if v.polyface_mesh_vertex_index4 != 0 {
                writer.write_code_pair(&CodePair::new_i16(
                    74,
                    v.polyface_mesh_vertex_index4 as i16,
                ))?;
            }
        }
        if version >= AcadVersion::R2010 {
            writer.write_code_pair(&CodePair::new_i32(91, v.identifier))?;
        }
        Ok(true)
    }
    fn post_write<T>(
        &self,
        version: AcadVersion,
        write_handles: bool,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<()>
    where
        T: Write + ?Sized,
    {
        match self.specific {
            EntityType::Attribute(ref att) => {
                self.write_attribute_m_text(att.m_text.clone(), version, write_handles, writer)?
            }
            EntityType::AttributeDefinition(ref att) => {
                self.write_attribute_m_text(att.m_text.clone(), version, write_handles, writer)?
            }
            EntityType::Insert(ref ins) => {
                for (a, att_handle) in &ins.__attributes_and_handles {
                    let a = Entity {
                        common: EntityCommon {
                            handle: *att_handle,
                            ..Default::default()
                        },
                        specific: EntityType::Attribute(a.clone()),
                    };
                    a.write(version, write_handles, writer)?;
                }
                Entity::write_seqend(version, write_handles, writer)?;
            }
            EntityType::Polyline(ref poly) => {
                for (v, vertex_handle) in &poly.__vertices_and_handles {
                    let mut v = v.clone();
                    v.set_is_3d_polyline_vertex(poly.get_is_3d_polyline());
                    v.set_is_3d_polygon_mesh(poly.get_is_3d_polygon_mesh());
                    let v = Entity {
                        common: EntityCommon {
                            handle: *vertex_handle,
                            ..Default::default()
                        },
                        specific: EntityType::Vertex(v),
                    };
                    v.write(version, write_handles, writer)?;
                }
                Entity::write_seqend(version, write_handles, writer)?;
            }
            _ => (),
        }

        Ok(())
    }
    fn write_attribute_m_text<T>(
        &self,
        m_text: MText,
        version: AcadVersion,
        write_handles: bool,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<()>
    where
        T: Write + ?Sized,
    {
        let m_text_common = EntityCommon {
            handle: Handle::empty(), // TODO: set handle
            __owner_handle: self.common.handle,
            is_in_paper_space: self.common.is_in_paper_space,
            layer: self.common.layer.clone(),
            ..Default::default()
        };
        let m_text = Entity {
            common: m_text_common,
            specific: EntityType::MText(m_text),
        };
        m_text.write(version, write_handles, writer)?;
        Ok(())
    }
    fn write_seqend<T>(
        version: AcadVersion,
        write_handles: bool,
        writer: &mut CodePairWriter<T>,
    ) -> DxfResult<()>
    where
        T: Write + ?Sized,
    {
        let seqend = Entity {
            common: Default::default(),
            specific: EntityType::Seqend(Default::default()),
        };
        seqend.write(version, write_handles, writer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::*;
    use crate::enums::*;
    use crate::helper_functions::tests::*;
    use crate::objects::*;
    use crate::*;

    fn read_entity(entity_type: &str, body: String) -> Entity {
        let drawing = from_section(
            "ENTITIES",
            vec!["0", entity_type, body.as_str()].join("\r\n").as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        entities[0].clone()
    }

    #[test]
    fn read_empty_entities_section() {
        let drawing = parse_drawing(
            vec!["0", "SECTION", "2", "ENTITIES", "0", "ENDSEC", "0", "EOF"]
                .join("\r\n")
                .as_str(),
        );
        let entities = drawing.entities();
        assert_eq!(0, entities.count());
    }

    #[test]
    fn read_unsupported_entity() {
        let drawing = parse_drawing(
            vec![
                "0",
                "SECTION",
                "2",
                "ENTITIES",
                "0",
                "UNSUPPORTED_ENTITY",
                "1",
                "unsupported string",
                "0",
                "ENDSEC",
                "0",
                "EOF",
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities();
        assert_eq!(0, entities.count());
    }

    #[test]
    fn read_unsupported_entity_between_supported_entities() {
        let drawing = parse_drawing(
            vec![
                "0",
                "SECTION",
                "2",
                "ENTITIES",
                "0",
                "LINE",
                "0",
                "UNSUPPORTED_ENTITY",
                "1",
                "unsupported string",
                "0",
                "CIRCLE",
                "0",
                "ENDSEC",
                "0",
                "EOF",
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a line"),
        }
        match entities[1].specific {
            EntityType::Circle(_) => (),
            _ => panic!("expected a circle"),
        }
    }

    #[test]
    fn read_entity_with_no_values() {
        let drawing = parse_drawing(
            vec![
                "0", "SECTION", "2", "ENTITIES", "0", "LINE", "0", "ENDSEC", "0", "EOF",
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a line"),
        }
    }

    #[test]
    fn read_common_entity_fields() {
        let ent = read_entity("LINE", vec!["8", "layer"].join("\r\n"));
        assert_eq!("layer", ent.common.layer);
    }

    #[test]
    fn read_line() {
        let ent = read_entity(
            "LINE",
            vec![
                "10", "1.1", // p1
                "20", "2.2", "30", "3.3", "11", "4.4", // p2
                "21", "5.5", "31", "6.6",
            ]
            .join("\r\n"),
        );
        match ent.specific {
            EntityType::Line(ref line) => {
                assert_eq!(Point::new(1.1, 2.2, 3.3), line.p1);
                assert_eq!(Point::new(4.4, 5.5, 6.6), line.p2);
            }
            _ => panic!("expected a line"),
        }
    }

    #[test]
    fn write_common_entity_fields() {
        let mut drawing = Drawing::new();
        let mut ent = Entity {
            common: Default::default(),
            specific: EntityType::Line(Default::default()),
        };
        ent.common.layer = "some-layer".to_owned();
        drawing.add_entity(ent);
        assert_contains(
            &drawing,
            vec![
                "  0",
                "LINE",
                "  5",
                "10",
                "100",
                "AcDbEntity",
                "  8",
                "some-layer",
            ]
            .join("\r\n"),
        );
    }

    #[test]
    fn write_specific_entity_fields() {
        let mut drawing = Drawing::new();
        let line = Line {
            p1: Point::new(1.1, 2.2, 3.3),
            p2: Point::new(4.4, 5.5, 6.6),
            ..Default::default()
        };
        drawing.add_entity(Entity::new(EntityType::Line(line)));
        assert_contains(
            &drawing,
            vec![
                "100", "AcDbLine", " 10", "1.1", " 20", "2.2", " 30", "3.3", " 11", "4.4", " 21",
                "5.5", " 31", "6.6",
            ]
            .join("\r\n"),
        );
    }

    #[test]
    fn read_multiple_entities() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                "0", "CIRCLE", "10", "1.1", // center
                "20", "2.2", "30", "3.3", "40", "4.4", // radius
                "0", "LINE", "10", "5.5", // p1
                "20", "6.6", "30", "7.7", "11", "8.8", // p2
                "21", "9.9", "31", "10.1",
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());

        // verify circle
        match entities[0].specific {
            EntityType::Circle(ref circle) => {
                assert_eq!(Point::new(1.1, 2.2, 3.3), circle.center);
                assert!(approx_eq!(f64, 4.4, circle.radius));
            }
            _ => panic!("expected a line"),
        }

        // verify line
        match entities[1].specific {
            EntityType::Line(ref line) => {
                assert_eq!(Point::new(5.5, 6.6, 7.7), line.p1);
                assert_eq!(Point::new(8.8, 9.9, 10.1), line.p2);
            }
            _ => panic!("expected a line"),
        }
    }

    #[test]
    fn read_field_with_multiples_common() {
        let ent = read_entity("LINE", vec!["310", "0102", "310", "0304"].join("\r\n"));
        assert_eq!(
            vec![vec![0x01, 0x02], vec![0x03, 0x04]],
            ent.common.preview_image_data
        );
    }

    #[test]
    fn write_field_with_multiples_common() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2000;
        drawing.add_entity(Entity {
            common: EntityCommon {
                preview_image_data: vec![vec![0x01, 0x02], vec![0x03, 0x04]],
                ..Default::default()
            },
            specific: EntityType::Line(Default::default()),
        });
        assert_contains(&drawing, vec!["310", "0102", "310", "0304"].join("\r\n"));
    }

    #[test]
    fn read_field_with_multiples_specific() {
        let ent = read_entity(
            "3DSOLID",
            vec!["1", "one-1", "1", "one-2", "3", "three-1", "3", "three-2"].join("\r\n"),
        );
        match ent.specific {
            EntityType::Solid3D(ref solid3d) => {
                assert_eq!(vec!["one-1", "one-2"], solid3d.custom_data);
                assert_eq!(vec!["three-1", "three-2"], solid3d.custom_data2);
            }
            _ => panic!("expected a 3DSOLID"),
        }
    }

    #[test]
    fn write_field_with_multiples_specific() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13; // 3DSOLID is only supported on R13+
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Solid3D(Solid3D {
                custom_data: vec![String::from("one-1"), String::from("one-2")],
                custom_data2: vec![String::from("three-1"), String::from("three-2")],
                ..Default::default()
            }),
        });
        assert_contains(
            &drawing,
            vec![
                "  1", "one-1", "  1", "one-2", "  3", "three-1", "  3", "three-2",
            ]
            .join("\r\n"),
        );
    }

    #[test]
    fn read_entity_with_post_parse() {
        let ent = read_entity(
            "IMAGE",
            vec![
                "14", "1.1", // clipping_vertices[0]
                "24", "2.2", "14", "3.3", // clipping_vertices[1]
                "24", "4.4", "14", "5.5", // clipping_vertices[2]
                "24", "6.6",
            ]
            .join("\r\n"),
        );
        match ent.specific {
            EntityType::Image(ref image) => {
                assert_eq!(3, image.clipping_vertices.len());
                assert_eq!(Point::new(1.1, 2.2, 0.0), image.clipping_vertices[0]);
                assert_eq!(Point::new(3.3, 4.4, 0.0), image.clipping_vertices[1]);
                assert_eq!(Point::new(5.5, 6.6, 0.0), image.clipping_vertices[2]);
            }
            _ => panic!("expected an IMAGE"),
        }
    }

    #[test]
    fn write_entity_with_write_order() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R14; // IMAGE is only supported on R14+
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Image(Image {
                clipping_vertices: vec![
                    Point::new(1.1, 2.2, 0.0),
                    Point::new(3.3, 4.4, 0.0),
                    Point::new(5.5, 6.6, 0.0),
                ],
                ..Default::default()
            }),
        });
        assert_contains(
            &drawing,
            vec![
                " 91",
                "        3",
                " 14",
                "1.1",
                " 24",
                "2.2",
                " 14",
                "3.3",
                " 24",
                "4.4",
                " 14",
                "5.5",
                " 24",
                "6.6",
            ]
            .join("\r\n"),
        );
    }

    #[test]
    fn read_entity_with_custom_reader_mtext() {
        let ent = read_entity(
            "MTEXT",
            vec![
                "50", "1.1", // rotation angle
                "75", "7", // column type
                "50", "3", // column count
                "50", "10", // column values
                "50", "20", "50", "30",
            ]
            .join("\r\n"),
        );
        match ent.specific {
            EntityType::MText(ref mtext) => {
                assert!(approx_eq!(f64, 1.1, mtext.rotation_angle));
                assert_eq!(7, mtext.column_type);
                assert_eq!(3, mtext.column_count);
                assert_eq!(3, mtext.column_heights.len());
                assert!(approx_eq!(f64, 10.0, mtext.column_heights[0]));
                assert!(approx_eq!(f64, 20.0, mtext.column_heights[1]));
                assert!(approx_eq!(f64, 30.0, mtext.column_heights[2]));
            }
            _ => panic!("expected an MTEXT"),
        }
    }

    #[test]
    fn read_entity_after_entity_with_custom_reader() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                "  0", "MTEXT", // has a custom reader
                "  0", "LINE", // uses the auto-generated reader
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::MText(_) => {}
            _ => panic!("expected an mtext"),
        }
        match entities[1].specific {
            EntityType::Line(_) => {}
            _ => panic!("expected a line"),
        }
    }

    #[test]
    fn read_entity_with_flags() {
        let ent = read_entity("IMAGE", vec!["70", "5"].join("\r\n"));
        match ent.specific {
            EntityType::Image(ref image) => {
                assert!(image.get_show_image());
                assert!(image.get_use_clipping_boundary());
            }
            _ => panic!("expected an IMAGE"),
        }
    }

    #[test]
    fn write_entity_with_flags() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R14; // IMAGE is only supported on R14+
        let mut image = Image::default();
        assert_eq!(0, image.display_options_flags);
        image.set_show_image(true);
        image.set_use_clipping_boundary(true);
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Image(image),
        });
        assert_contains(
            &drawing,
            vec![
                " 70", "     5", // flags
                "280", "     1", // sentinels to make sure we're not reading a header value
                "281", "    50",
            ]
            .join("\r\n"),
        );
    }

    #[test]
    fn read_entity_with_handle_and_pointer() {
        let ent = read_entity(
            "3DSOLID",
            vec![
                "5", "A1", // handle
                "330", "A2", // owner handle
                "350", "A3", // history_object pointer
            ]
            .join("\r\n"),
        );
        assert_eq!(Handle(0xa1), ent.common.handle);
        assert_eq!(Handle(0xa2), ent.common.__owner_handle);
        match ent.specific {
            EntityType::Solid3D(ref solid) => {
                assert_eq!(Handle(0xa3), solid.__history_object_handle)
            }
            _ => panic!("expected a 3DSOLID entity"),
        }
    }

    #[test]
    fn write_entity_with_handle_and_pointer() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2000;
        drawing.add_entity(Entity {
            common: EntityCommon {
                __owner_handle: Handle(0xa2),
                ..Default::default()
            },
            specific: EntityType::Line(Default::default()),
        });
        assert_contains(
            &drawing,
            vec!["  0", "LINE", "  5", "10", "330", "A2"].join("\r\n"),
        );
    }

    #[test]
    fn write_version_specific_entity() {
        let mut drawing = Drawing::new();
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Solid3D(Default::default()),
        });

        // 3DSOLID not supported in R12 and below
        drawing.header.version = AcadVersion::R12;
        assert_contains(
            &drawing,
            vec!["  0", "SECTION", "  2", "ENTITIES", "  0", "ENDSEC"].join("\r\n"),
        );

        // but it is in R13 and above
        drawing.header.version = AcadVersion::R13;
        assert_contains(
            &drawing,
            vec!["  0", "SECTION", "  2", "ENTITIES", "  0", "3DSOLID"].join("\r\n"),
        );
    }

    #[test]
    fn read_polyline() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                "  0", "POLYLINE", // polyline sentinel
                "  0", "VERTEX", // vertex 1
                " 10", "1.1", " 20", "2.1", " 30", "3.1", "  0", "VERTEX", // vertex 2
                " 10", "1.2", " 20", "2.2", " 30", "3.2", "  0", "VERTEX", // vertex 3
                " 10", "1.3", " 20", "2.3", " 30", "3.3", "  0", "SEQEND", // end sequence
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => {
                assert_eq!(
                    vec![
                        Vertex {
                            location: Point::new(1.1, 2.1, 3.1),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.2, 2.2, 3.2),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.3, 2.3, 3.3),
                            ..Default::default()
                        },
                    ],
                    poly.vertices().cloned().collect::<Vec<_>>()
                );
            }
            _ => panic!("expected a POLYLINE"),
        }
    }

    #[test]
    fn read_polyline_without_seqend() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                "  0", "POLYLINE", // polyline sentinel
                "  0", "VERTEX", // vertex 1
                " 10", "1.1", " 20", "2.1", " 30", "3.1", "  0", "VERTEX", // vertex 2
                " 10", "1.2", " 20", "2.2", " 30", "3.2", "  0", "VERTEX", // vertex 3
                " 10", "1.3", " 20", "2.3", " 30", "3.3",
                // no end sequence
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => {
                assert_eq!(
                    vec![
                        Vertex {
                            location: Point::new(1.1, 2.1, 3.1),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.2, 2.2, 3.2),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.3, 2.3, 3.3),
                            ..Default::default()
                        },
                    ],
                    poly.vertices().cloned().collect::<Vec<_>>()
                );
            }
            _ => panic!("expected a POLYLINE"),
        }
    }

    #[test]
    fn read_empty_polyline() {
        let drawing = from_section(
            "ENTITIES",
            vec!["0", "POLYLINE", "0", "SEQEND"].join("\r\n").as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => assert_eq!(0, poly.vertices().count()),
            _ => panic!("expected a POLYLINE"),
        }
    }

    #[test]
    fn read_empty_polyline_without_seqend() {
        let drawing = from_section("ENTITIES", vec!["0", "POLYLINE"].join("\r\n").as_str());
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => assert_eq!(0, poly.vertices().count()),
            _ => panic!("expected a POLYLINE"),
        }
    }

    #[test]
    fn read_polyline_with_trailing_entity() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                "  0", "POLYLINE", // polyline sentinel
                "  0", "VERTEX", // vertex 1
                " 10", "1.1", " 20", "2.1", " 30", "3.1", "  0", "VERTEX", // vertex 2
                " 10", "1.2", " 20", "2.2", " 30", "3.2", "  0", "VERTEX", // vertex 3
                " 10", "1.3", " 20", "2.3", " 30", "3.3", "  0", "SEQEND", // end sequence
                "  0", "LINE", // trailing entity
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => {
                assert_eq!(
                    vec![
                        Vertex {
                            location: Point::new(1.1, 2.1, 3.1),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.2, 2.2, 3.2),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.3, 2.3, 3.3),
                            ..Default::default()
                        },
                    ],
                    poly.vertices().cloned().collect::<Vec<_>>()
                );
            }
            _ => panic!("expected a POLYLINE"),
        }

        match entities[1].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a LINE"),
        }
    }

    #[test]
    fn read_polyline_without_seqend_with_trailing_entity() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                "  0", "POLYLINE", // polyline sentinel
                "  0", "VERTEX", // vertex 1
                " 10", "1.1", " 20", "2.1", " 30", "3.1", "  0", "VERTEX", // vertex 2
                " 10", "1.2", " 20", "2.2", " 30", "3.2", "  0", "VERTEX", // vertex 3
                " 10", "1.3", " 20", "2.3", " 30", "3.3", // no end sequence
                "  0", "LINE", // trailing entity
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => {
                assert_eq!(
                    vec![
                        Vertex {
                            location: Point::new(1.1, 2.1, 3.1),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.2, 2.2, 3.2),
                            ..Default::default()
                        },
                        Vertex {
                            location: Point::new(1.3, 2.3, 3.3),
                            ..Default::default()
                        },
                    ],
                    poly.vertices().cloned().collect::<Vec<_>>()
                );
            }
            _ => panic!("expected a POLYLINE"),
        }

        match entities[1].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a LINE"),
        }
    }

    #[test]
    fn read_empty_polyline_with_trailing_entity() {
        let drawing = from_section(
            "ENTITIES",
            vec!["0", "POLYLINE", "0", "SEQEND", "0", "LINE"]
                .join("\r\n")
                .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => assert_eq!(0, poly.vertices().count()),
            _ => panic!("expected a POLYLINE"),
        }
        match entities[1].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a LINE"),
        }
    }

    #[test]
    fn read_empty_polyline_without_seqend_with_trailing_entity() {
        let drawing = from_section(
            "ENTITIES",
            vec!["0", "POLYLINE", "0", "LINE"].join("\r\n").as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Polyline(ref poly) => assert_eq!(0, poly.vertices().count()),
            _ => panic!("expected a POLYLINE"),
        }
        match entities[1].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a LINE"),
        }
    }

    #[test]
    fn write_2d_polyline() {
        let mut drawing = Drawing::new();
        let mut poly = Polyline::default();
        poly.add_vertex(
            &mut drawing,
            Vertex {
                location: Point::new(1.1, 2.1, 3.1),
                ..Default::default()
            },
        );
        poly.add_vertex(
            &mut drawing,
            Vertex {
                location: Point::new(1.2, 2.2, 3.2),
                ..Default::default()
            },
        );
        poly.add_vertex(
            &mut drawing,
            Vertex {
                location: Point::new(1.3, 2.3, 3.3),
                ..Default::default()
            },
        );
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Polyline(poly),
        });
        assert_contains(
            &drawing,
            vec![
                "  0",
                "POLYLINE", // polyline
                "  5",
                "13",
                "100",
                "AcDbEntity",
                "  8",
                "0",
                "100",
                "AcDb2dPolyline",
                " 66",
                "     1",
                " 10",
                "0.0",
                " 20",
                "0.0",
                " 30",
                "0.0",
                "  0",
                "VERTEX", // vertex 1
                "  5",
                "10",
                "100",
                "AcDbEntity",
                "  8",
                "0",
                "100",
                "AcDbVertex",
                "100",
                "AcDb2dVertex",
                " 10",
                "1.1",
                " 20",
                "2.1",
                " 30",
                "3.1",
                " 70",
                "     0",
                " 50",
                "0.0",
                "  0",
                "VERTEX", // vertex 2
                "  5",
                "11",
                "100",
                "AcDbEntity",
                "  8",
                "0",
                "100",
                "AcDbVertex",
                "100",
                "AcDb2dVertex",
                " 10",
                "1.2",
                " 20",
                "2.2",
                " 30",
                "3.2",
                " 70",
                "     0",
                " 50",
                "0.0",
                "  0",
                "VERTEX", // vertex 3
                "  5",
                "12",
                "100",
                "AcDbEntity",
                "  8",
                "0",
                "100",
                "AcDbVertex",
                "100",
                "AcDb2dVertex",
                " 10",
                "1.3",
                " 20",
                "2.3",
                " 30",
                "3.3",
                " 70",
                "     0",
                " 50",
                "0.0",
                "  0",
                "SEQEND", // end sequence
            ]
            .join("\r\n"),
        );
    }

    #[test]
    fn write_3d_polyline() {
        let mut drawing = Drawing::new();
        let mut poly = Polyline::default();
        poly.add_vertex(
            &mut drawing,
            Vertex {
                location: Point::new(1.1, 2.1, 3.1),
                ..Default::default()
            },
        );
        poly.set_is_3d_polyline(true);
        drawing.add_entity(Entity {
            common: Default::default(),
            specific: EntityType::Polyline(poly),
        });
        assert_contains(
            &drawing,
            vec![
                "  0",
                "POLYLINE", // polyline
                "  5",
                "11",
                "100",
                "AcDbEntity",
                "  8",
                "0",
                "100",
                "AcDb3dPolyline", // 3d = true
                " 66",
                "     1",
                " 10",
                "0.0",
                " 20",
                "0.0",
                " 30",
                "0.0",
                " 70",
                "     8", // 3d = true
                "  0",
                "VERTEX", // vertex 1
                "  5",
                "10",
                "100",
                "AcDbEntity",
                "  8",
                "0",
                "100",
                "AcDbVertex",
                "100",
                "AcDb3dPolylineVertex", // 3d = true
                " 10",
                "1.1",
                " 20",
                "2.1",
                " 30",
                "3.1",
                " 70",
                "    32", // 3d = true
            ]
            .join("\r\n"),
        );
    }

    #[test]
    fn read_lw_polyline_with_no_vertices() {
        let drawing = from_section(
            "ENTITIES",
            vec!["0", "LWPOLYLINE", "43", "43.0"].join("\r\n").as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::LwPolyline(ref poly) => {
                assert!(approx_eq!(f64, 43.0, poly.constant_width));
                assert_eq!(0, poly.vertices.len());
            }
            _ => panic!("expected an LWPOLYLINE"),
        }
    }

    #[test]
    fn read_lw_polyline_with_one_vertex() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                "0",
                "LWPOLYLINE",
                "43",
                "43.0",
                // vertex 1
                "10",
                "1.1",
                "20",
                "2.1",
                "40",
                "40.1",
                "41",
                "41.1",
                "42",
                "42.1",
                "91",
                "91",
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::LwPolyline(ref poly) => {
                assert!(approx_eq!(f64, 43.0, poly.constant_width));
                assert_eq!(1, poly.vertices.len());
                // vertex 1
                assert!(approx_eq!(f64, 1.1, poly.vertices[0].x));
                assert!(approx_eq!(f64, 2.1, poly.vertices[0].y));
                assert!(approx_eq!(f64, 40.1, poly.vertices[0].starting_width));
                assert!(approx_eq!(f64, 41.1, poly.vertices[0].ending_width));
                assert!(approx_eq!(f64, 42.1, poly.vertices[0].bulge));
                assert_eq!(91, poly.vertices[0].id);
            }
            _ => panic!("expected an LWPOLYLINE"),
        }
    }

    #[test]
    fn read_lw_polyline_with_multiple_vertices() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                "0",
                "LWPOLYLINE",
                "43",
                "43.0",
                // vertex 1
                "10",
                "1.1",
                "20",
                "2.1",
                "40",
                "40.1",
                "41",
                "41.1",
                "42",
                "42.1",
                "91",
                "91",
                // vertex 2
                "10",
                "1.2",
                "20",
                "2.2",
                "40",
                "40.2",
                "41",
                "41.2",
                "42",
                "42.2",
                "91",
                "92",
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::LwPolyline(ref poly) => {
                assert!(approx_eq!(f64, 43.0, poly.constant_width));
                assert_eq!(2, poly.vertices.len());
                // vertex 1
                assert!(approx_eq!(f64, 1.1, poly.vertices[0].x));
                assert!(approx_eq!(f64, 2.1, poly.vertices[0].y));
                assert!(approx_eq!(f64, 40.1, poly.vertices[0].starting_width));
                assert!(approx_eq!(f64, 41.1, poly.vertices[0].ending_width));
                assert!(approx_eq!(f64, 42.1, poly.vertices[0].bulge));
                assert_eq!(91, poly.vertices[0].id);
                // vertex 2
                assert!(approx_eq!(f64, 1.2, poly.vertices[1].x));
                assert!(approx_eq!(f64, 2.2, poly.vertices[1].y));
                assert!(approx_eq!(f64, 40.2, poly.vertices[1].starting_width));
                assert!(approx_eq!(f64, 41.2, poly.vertices[1].ending_width));
                assert!(approx_eq!(f64, 42.2, poly.vertices[1].bulge));
                assert_eq!(92, poly.vertices[1].id);
            }
            _ => panic!("expected an LWPOLYLINE"),
        }
    }

    #[test]
    fn write_lw_polyline() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2013;
        let mut poly = LwPolyline::default();
        poly.constant_width = 43.0;
        poly.vertices.push(LwPolylineVertex {
            x: 1.1,
            y: 2.1,
            ..Default::default()
        });
        poly.vertices.push(LwPolylineVertex {
            x: 1.2,
            y: 2.2,
            starting_width: 40.2,
            ending_width: 41.2,
            bulge: 42.2,
            id: 92,
        });
        drawing.add_entity(Entity::new(EntityType::LwPolyline(poly)));
        assert_contains(
            &drawing,
            vec![
                "100",
                "AcDbPolyline",
                " 90",
                "        2",
                " 70",
                "     0",
                " 43",
                "43.0",
                // vertex 1
                " 10",
                "1.1",
                " 20",
                "2.1",
                " 91",
                "        0",
                // vertex 2
                " 10",
                "1.2",
                " 20",
                "2.2",
                " 91",
                "       92",
                " 40",
                "40.2",
                " 41",
                "41.2",
                " 42",
                "42.2",
            ]
            .join("\r\n"),
        );
    }

    #[test]
    fn read_dimension() {
        let ent = read_entity(
            "DIMENSION",
            vec![
                "1",
                "text",
                "100",
                "AcDbOrdinateDimension",
                "13",
                "1.1", // definition_point_2
                "23",
                "2.2",
                "33",
                "3.3",
                "14",
                "4.4", // definition_point_3
                "24",
                "5.5",
                "34",
                "6.6",
            ]
            .join("\r\n"),
        );
        match ent.specific {
            EntityType::OrdinateDimension(ref dim) => {
                assert_eq!("text", dim.dimension_base.text);
                assert_eq!(Point::new(1.1, 2.2, 3.3), dim.definition_point_2);
                assert_eq!(Point::new(4.4, 5.5, 6.6), dim.definition_point_3);
            }
            _ => panic!("expected an ordinate dimension"),
        }
    }

    #[test]
    fn read_entity_after_unsupported_dimension() {
        let drawing = from_section(
            "ENTITIES",
            vec![
                "0",
                "DIMENSION",
                "1",
                "text",
                "100",
                "AcDbSomeUnsupportedDimensionType",
                "10",
                "1.1",
                "20",
                "2.2",
                "30",
                "3.3",
                "0",
                "LINE",
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Line(_) => {}
            _ => panic!("expected a line"),
        }
    }

    #[test]
    fn write_dimension() {
        let dim = RadialDimension {
            dimension_base: DimensionBase {
                text: String::from("some-text"),
                ..Default::default()
            },
            definition_point_2: Point::new(1.1, 2.2, 3.3),
            ..Default::default()
        };
        let ent = Entity::new(EntityType::RadialDimension(dim));
        let mut drawing = Drawing::new();
        drawing.add_entity(ent);
        assert_contains(&drawing, vec!["  0", "DIMENSION"].join("\r\n"));
        assert_contains(&drawing, vec!["  1", "some-text"].join("\r\n"));
        assert_contains(
            &drawing,
            vec![
                "100",
                "AcDbRadialDimension",
                " 15",
                "1.1", // definition_point_2
                " 25",
                "2.2",
                " 35",
                "3.3",
                " 40",
                "0.0", // leader_length
            ]
            .join("\r\n"),
        );
    }

    #[test]
    fn read_insert_with_separate_attributes() {
        let file = from_section(
            "ENTITIES",
            vec![
                "  0", "INSERT", " 66", "0", // no attributes
                "  0",
                "ATTRIB", // this is a separate attribute, not tied to the `INSERT` entity
                "  0",
                "SEQEND", // this is a separate `SEQEND` entity, not tied to the `INSERT` entity
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = file.entities().collect::<Vec<_>>();
        assert_eq!(3, entities.len());
        match entities[0].specific {
            EntityType::Insert(_) => (),
            _ => panic!("expected an INSERT"),
        }
        match entities[1].specific {
            EntityType::Attribute(_) => (),
            _ => panic!("expected an ATTRIB"),
        }
        match entities[2].specific {
            EntityType::Seqend(_) => (),
            _ => panic!("expected a SEQEND"),
        }
    }

    #[test]
    fn read_insert_with_embedded_attributes() {
        let file = from_section(
            "ENTITIES",
            vec![
                "  0", "INSERT", " 66", "1", // includes attributes
                "  0", "ATTRIB", // these are embedded attributes tied to the `INSERT` entity
                "  0", "ATTRIB", "  0",
                "SEQEND", // this is an embedded `SEQEND` entity tied to the `INSERT` entity
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = file.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Insert(ref ins) => assert_eq!(2, ins.attributes().count()),
            _ => panic!("exepcted an INSERT"),
        }
    }

    #[test]
    fn write_insert_with_embedded_attributes() {
        let mut drawing = Drawing::new();
        let mut ins = Insert::default();
        ins.add_attribute(&mut drawing, Attribute::default());
        let ent = Entity::new(EntityType::Insert(ins));
        drawing.add_entity(ent);
        assert_contains(&drawing, vec!["  0", "INSERT"].join("\r\n"));
        assert_contains(
            &drawing,
            vec![
                "100",
                "AcDbBlockReference",
                " 66",
                "     1", // contains attributes
            ]
            .join("\r\n"),
        );
        assert_contains(&drawing, vec!["  0", "ATTRIB"].join("\r\n"));
        assert_contains(&drawing, vec!["  0", "SEQEND"].join("\r\n"));
    }

    #[test]
    fn round_trip_insert_with_attributes() {
        let mut drawing = Drawing::new();
        let mut ins = Insert::default();
        ins.add_attribute(&mut drawing, Attribute::default());
        let ent = Entity::new(EntityType::Insert(ins));
        drawing.add_entity(ent);

        let drawing = parse_drawing(&to_test_string(&drawing));
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Insert(ref ins) => assert_eq!(1, ins.attributes().count()),
            _ => panic!("expected an INSERT"),
        }
    }

    #[test]
    fn read_attribute_with_attached_mtext() {
        let file = from_section(
            "ENTITIES",
            vec!["  0", "ATTRIB", "  0", "MTEXT", "  1", "m_text"]
                .join("\r\n")
                .as_str(),
        );
        let entities = file.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Attribute(ref att) => assert_eq!("m_text", att.m_text.text),
            _ => panic!("exepcted an attribute"),
        }
    }

    #[test]
    fn write_attribute_with_attached_mtext() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13; // MTEXT is only written on R13+
        drawing.add_entity(Entity::new(EntityType::Attribute(Default::default())));
        assert_contains(&drawing, vec!["  0", "ATTRIB"].join("\r\n"));
        assert_contains(&drawing, vec!["  0", "MTEXT"].join("\r\n"));
    }

    #[test]
    fn round_trip_attribute_with_attached_mtext() {
        let att = Attribute {
            m_text: MText {
                text: String::from("m_text"),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R13; // MTEXT is only written on R13+
        drawing.add_entity(Entity::new(EntityType::Attribute(att)));

        let drawing = parse_drawing(&to_test_string(&drawing));
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(1, entities.len());
        match entities[0].specific {
            EntityType::Attribute(ref att) => assert_eq!("m_text", att.m_text.text),
            _ => panic!("expected a attribute"),
        }
    }

    #[test]
    fn read_extension_data() {
        let ent = read_entity(
            "LINE",
            vec!["102", "{IXMILIA", "  1", "some string", "102", "}"].join("\r\n"),
        );
        assert_eq!(1, ent.common.extension_data_groups.len());
        let group = &ent.common.extension_data_groups[0];
        assert_eq!("IXMILIA", group.application_name);
        match group.items[0] {
            ExtensionGroupItem::CodePair(ref p) => {
                assert_eq!(&CodePair::new_str(1, "some string"), p)
            }
            _ => panic!("expected a code pair"),
        }
    }

    #[test]
    fn write_extension_data() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R14;
        drawing.add_entity(Entity {
            common: EntityCommon {
                extension_data_groups: vec![ExtensionGroup {
                    application_name: String::from("IXMILIA"),
                    items: vec![ExtensionGroupItem::CodePair(CodePair::new_str(
                        1,
                        "some string",
                    ))],
                }],
                ..Default::default()
            },
            specific: EntityType::Line(Line::default()),
        });
        assert_contains(
            &drawing,
            vec!["102", "{IXMILIA", "  1", "some string", "102", "}"].join("\r\n"),
        );
    }

    #[test]
    fn read_x_data() {
        let ent = read_entity(
            "LINE",
            vec!["1001", "IXMILIA", "1000", "some string"].join("\r\n"),
        );
        assert_eq!(1, ent.common.x_data.len());
        let x = &ent.common.x_data[0];
        assert_eq!("IXMILIA", x.application_name);
        match x.items[0] {
            XDataItem::Str(ref s) => assert_eq!("some string", s),
            _ => panic!("expected a string"),
        }
    }

    #[test]
    fn write_x_data() {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2000;
        drawing.add_entity(Entity {
            common: EntityCommon {
                x_data: vec![XData {
                    application_name: String::from("IXMILIA"),
                    items: vec![XDataItem::Real(1.1)],
                }],
                ..Default::default()
            },
            specific: EntityType::Line(Line::default()),
        });
        assert_contains(
            &drawing,
            vec![
                "1001", "IXMILIA", "1040", "1.1", "  0",
                "ENDSEC", // xdata is written after all the entity's other code pairs
            ]
            .join("\r\n"),
        );
    }

    #[test]
    fn read_entity_after_extension_data() {
        let drawing = parse_drawing(
            vec![
                "  0", "SECTION", "  2", "ENTITIES", "  0", "LINE", "102", "{IXMILIA", "102", "}",
                "  0", "CIRCLE", "  0", "ENDSEC", "  0", "EOF",
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a line"),
        }
        match entities[1].specific {
            EntityType::Circle(_) => (),
            _ => panic!("expected a circle"),
        }
    }

    #[test]
    fn read_entity_after_x_data() {
        let drawing = parse_drawing(
            vec![
                "  0", "SECTION", "  2", "ENTITIES", "  0", "LINE", "1001", "IXMILIA", "  0",
                "CIRCLE", "  0", "ENDSEC", "  0", "EOF",
            ]
            .join("\r\n")
            .as_str(),
        );
        let entities = drawing.entities().collect::<Vec<_>>();
        assert_eq!(2, entities.len());
        match entities[0].specific {
            EntityType::Line(_) => (),
            _ => panic!("expected a line"),
        }
        match entities[1].specific {
            EntityType::Circle(_) => (),
            _ => panic!("expected a circle"),
        }
    }

    #[test]
    fn read_all_types() {
        for (type_string, subclass, expected_type, _) in all_types::get_all_entity_types() {
            println!("parsing {}/{}", type_string, subclass);
            let ent = read_entity(
                type_string,
                vec![
                    "100",
                    subclass,
                    "102",
                    "{IXMILIA", // read extension data
                    "  1",
                    "some string",
                    "102",
                    "}",
                    "1001",
                    "IXMILIA", // read x data
                    "1040",
                    "1.1",
                ]
                .join("\r\n"),
            );

            // validate specific
            assert_eq!(expected_type, ent.specific);

            // validate extension data
            assert_eq!(1, ent.common.extension_data_groups.len());
            assert_eq!(
                "IXMILIA",
                ent.common.extension_data_groups[0].application_name
            );
            assert_eq!(1, ent.common.extension_data_groups[0].items.len());
            assert_eq!(
                ExtensionGroupItem::CodePair(CodePair::new_str(1, "some string")),
                ent.common.extension_data_groups[0].items[0]
            );

            // validate x data
            assert_eq!(1, ent.common.x_data.len());
            assert_eq!("IXMILIA", ent.common.x_data[0].application_name);
            assert_eq!(1, ent.common.x_data[0].items.len());
            assert_eq!(XDataItem::Real(1.1), ent.common.x_data[0].items[0]);
        }
    }

    #[test]
    fn write_all_types() {
        for (type_string, _, expected_type, max_version) in all_types::get_all_entity_types() {
            println!("writing {}", type_string);
            let mut common = EntityCommon::default();
            common.extension_data_groups.push(ExtensionGroup {
                application_name: String::from("IXMILIA"),
                items: vec![ExtensionGroupItem::CodePair(CodePair::new_str(
                    1,
                    "some string",
                ))],
            });
            common.x_data.push(XData {
                application_name: String::from("IXMILIA"),
                items: vec![XDataItem::Real(1.1)],
            });
            let mut drawing = Drawing::new();
            drawing.header.version = max_version;
            drawing.add_entity(Entity {
                common,
                specific: expected_type,
            });
            // 3DLINE writes as a LINE
            let type_string = if type_string == "3DLINE" {
                "LINE"
            } else {
                type_string
            };
            assert_contains(&drawing, vec!["  0", type_string].join("\r\n"));
            if max_version >= AcadVersion::R14 {
                // only written on R14+
                assert_contains(
                    &drawing,
                    vec!["102", "{IXMILIA", "  1", "some string", "102", "}"].join("\r\n"),
                );
            }
            if max_version >= AcadVersion::R2000 {
                // only written on R2000+
                assert_contains(
                    &drawing,
                    vec!["1001", "IXMILIA", "1040", "1.1"].join("\r\n"),
                );
            }
        }
    }

    #[test]
    fn normalize_mline_styles() {
        let mut file = Drawing::new();
        file.clear();
        let objects = file.objects().collect::<Vec<_>>();
        assert_eq!(0, objects.len());
        let mut mline = MLine::default();
        mline.style_name = String::from("style name");
        file.add_entity(Entity::new(EntityType::MLine(mline)));
        file.normalize();
        let objects = file.objects().collect::<Vec<_>>();
        assert_eq!(1, objects.len());
        match objects[0].specific {
            ObjectType::MLineStyle(ref ml) => assert_eq!("style name", ml.style_name),
            _ => panic!("expected an mline style"),
        }
    }

    #[test]
    fn normalize_dimension_styles() {
        let mut file = Drawing::new();
        file.clear();
        assert_eq!(0, file.dim_styles().count());
        file.add_entity(Entity::new(EntityType::RadialDimension(RadialDimension {
            dimension_base: DimensionBase {
                dimension_style_name: String::from("style name"),
                ..Default::default()
            },
            ..Default::default()
        })));
        file.normalize();
        let dim_styles = file.dim_styles().collect::<Vec<_>>();
        assert_eq!(3, dim_styles.len());
        assert_eq!("ANNOTATIVE", dim_styles[0].name);
        assert_eq!("STANDARD", dim_styles[1].name);
        assert_eq!("style name", dim_styles[2].name);
    }
}
