/**
  * Copyright 2014 Dropbox, Inc.
  *
  * Licensed under the Apache License, Version 2.0 (the "License");
  * you may not use this file except in compliance with the License.
  * You may obtain a copy of the License at
  *
  *    http://www.apache.org/licenses/LICENSE-2.0
  *
  * Unless required by applicable law or agreed to in writing, software
  * distributed under the License is distributed on an "AS IS" BASIS,
  * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  * See the License for the specific language governing permissions and
  * limitations under the License.
  */

package djinni

import djinni.ast._
import djinni.generatorTools._
import djinni.meta._

class SwiftMarshal(spec: Spec) extends Marshal(spec) {

  val swiftTypeIdent: IdentConverter = spec.swiftTypeIdentStyle

  override def typename(tm: MExpr): String = toSwiftType(tm)
  override def typename(name: String, ty: TypeDef): String = swiftTypeIdent(name)

  override def fqTypename(tm: MExpr): String = toSwiftType(tm)
  def fqTypename(name: String, ty: TypeDef): String = swiftTypeIdent(name)

  override def paramType(tm: MExpr): String = toSwiftType(tm)
  override def fqParamType(tm: MExpr): String = toSwiftType(tm)

  override def returnType(ret: Option[TypeRef]): String = ret.fold("Void")(ty => toSwiftType(ty.resolved))
  override def fqReturnType(ret: Option[TypeRef]): String = returnType(ret)

  override def fieldType(tm: MExpr): String = toSwiftType(tm)
  override def fqFieldType(tm: MExpr): String = toSwiftType(tm)

  override def toCpp(tm: MExpr, expr: String): String = throw new AssertionError("direct swift to cpp conversion not possible")
  override def fromCpp(tm: MExpr, expr: String): String = throw new AssertionError("direct cpp to swift conversion not possible")

  // Returns the Swift module import needed for a Meta type, or empty string if none
  def importForMeta(m: Meta): String = m match {
    case e: MExtern =>
      if (e.swift.module.nonEmpty) e.swift.module
      else ""
    case _ => ""
  }

  // Check if a Meta type requires UIKit import (CGFloat usage)
  def needsUIKit(m: Meta): Boolean = m match {
    case p: MPrimitive if p.idlName == "f32" => true // CGFloat requires UIKit
    case e: MExtern => e.swift.module == "UIKit"
    case _ => false
  }

  // Check if an extern type should be skipped in Swift
  def shouldSkip(m: Meta): Boolean = m match {
    case e: MExtern => e.swift.skip
    case _ => false
  }

  // Check if the base of an MExpr should be skipped
  def shouldSkipField(tm: MExpr): Boolean = {
    def checkBase(m: Meta): Boolean = m match {
      case e: MExtern => e.swift.skip
      case _: MProtobuf => true // Protobuf types are not supported in Swift generation
      case _ => false
    }
    tm.base match {
      case MOptional | MList | MSet | MArray => tm.args.headOption.exists(arg => checkBase(arg.base))
      case MMap => tm.args.exists(arg => checkBase(arg.base))
      case other => checkBase(other)
    }
  }

  // Check if an MExpr contains any extern types (which may not conform to Equatable/Hashable/Codable)
  def hasExternType(tm: MExpr): Boolean = {
    tm.base match {
      case _: MExtern => true
      case MOptional | MList | MSet | MMap | MArray => tm.args.exists(hasExternType)
      case _ => false
    }
  }

  private def toSwiftType(tm: MExpr): String = {
    def f(tm: MExpr): String = {
      tm.base match {
        case MOptional =>
          assert(tm.args.size == 1)
          f(tm.args.head) + "?"
        case MList =>
          assert(tm.args.size == 1)
          "[" + f(tm.args.head) + "]"
        case MSet =>
          assert(tm.args.size == 1)
          "Set<" + f(tm.args.head) + ">"
        case MMap =>
          assert(tm.args.size == 2)
          "[" + f(tm.args.head) + ": " + f(tm.args(1)) + "]"
        case MArray =>
          assert(tm.args.size == 1)
          "[" + f(tm.args.head) + "]"
        case e: MExtern =>
          if (e.swift.typename.nonEmpty) e.swift.typename
          else swiftTypeIdent(e.name)
        case o =>
          val base = o match {
            case p: MPrimitive => p.idlName match {
              case "bool" => "Bool"
              case "i8" => "Int8"
              case "i16" => "Int16"
              case "i32" => "Int32"
              case "i64" => "Int64"
              case "f32" => "CGFloat"
              case "f64" => "Double"
              case _ => throw new AssertionError("Unknown primitive: " + p.idlName)
            }
            case MString => "String"
            case MDate => "Date"
            case MBinary => "Data"
            case MVoid => "Void"
            case d: MDef => swiftTypeIdent(d.name)
            case p: MParam => p.name
            case _: MProtobuf => throw new AssertionError("Protobuf types should be filtered before reaching Swift marshal")
            case _ => throw new AssertionError("Unexpected type in Swift marshal: " + o)
          }
          base
      }
    }
    f(tm)
  }
}
