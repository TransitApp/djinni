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
  *
  * This file has been modified by Snap, Inc.
  */

package djinni

import djinni.ast.Record.DerivingType
import djinni.ast._
import djinni.generatorTools._
import djinni.meta._
import djinni.writer.IndentWriter

import scala.collection.mutable

class KotlinGenerator(spec: Spec) extends Generator(spec) {

  val javaAnnotationHeader = spec.javaAnnotation.map(pkg => '@' + pkg.split("\\.").last)
  val javaClassAccessModifierString = JavaAccessModifier.getKotlinCodeGenerationString(spec.javaClassAccessModifier)
  val marshal = new KotlinMarshal(spec, true)

  class JavaRefs() {
    var java = mutable.TreeSet[String]()

    spec.javaAnnotation.foreach(pkg => java.add(pkg))

    def find(ty: TypeRef) { find(ty.resolved) }
    def find(tm: MExpr) {
      tm.args.foreach(find)
      find(tm.base)
    }
    def find(m: Meta) = for(r <- marshal.references(m)) r match {
      case ImportRef(arg) => java.add(arg)
      case _ =>
    }
  }

  def writeJavaFile(ident: String, origin: String, refs: Iterable[String], f: IndentWriter => Unit) {
    createFile(spec.kotlinOutFolder.get, idJava.ty(ident) + ".kt", (w: IndentWriter) => {
      w.wl("// AUTOGENERATED FILE - DO NOT MODIFY!")
      w.wl("// This file was generated by Djinni from " + origin)
      w.wl
      spec.javaPackage.foreach(s => w.wl(s"package $s").wl)
      if (refs.nonEmpty) {
        refs.foreach(s => w.wl(s"import $s"))
        w.wl
      }
      f(w)
    })
  }

  def generateJavaConstants(w: IndentWriter, consts: Seq[Const]) = {

    def writeJavaConst(w: IndentWriter, ty: TypeRef, v: Any): Unit = v match {
      case l: Long if marshal.fieldType(ty).equalsIgnoreCase("long?") => w.w(l.toString + "L")
      case l: Long if marshal.fieldType(ty).equalsIgnoreCase("long") => w.w(l.toString + "L")
      case l: Long => w.w(l.toString)
      case d: Double if marshal.fieldType(ty).equalsIgnoreCase("float?") => w.w(d.toString + "F")
      case d: Double if marshal.fieldType(ty).equalsIgnoreCase("float") => w.w(d.toString + "F")
      case d: Double => w.w(d.toString)
      case b: Boolean => w.w(if (b) "true" else "false")
      case s: String => w.w(s)
      case e: EnumValue =>  w.w(s"${marshal.typename(ty)}.${idJava.enum(e)}")
      case v: ConstRef => w.w(idJava.const(v))
      case z: Map[_, _] => { // Value is record
        val recordMdef = ty.resolved.base.asInstanceOf[MDef]
        val record = recordMdef.body.asInstanceOf[Record]
        val vMap = z.asInstanceOf[Map[String, Any]]
        w.wl(s" ${marshal.typename(ty)}(")
        w.increase()
        // Use exact sequence
        val skipFirst = SkipFirst()
        for (f <- record.fields) {
          skipFirst {w.wl(",")}
          writeJavaConst(w, f.ty, vMap.apply(f.ident.name))
          w.w(" /* " + idJava.field(f.ident) + " */ ")
        }
        w.w(")")
        w.decrease()
      }
    }

    if (consts.nonEmpty) {
      w.w("companion object").braced {
        for (c <- consts) {
          writeDoc(w, c.doc)
          javaAnnotationHeader.foreach(w.wl)
          w.w(s"val ${idJava.const(c.ident)}: ${marshal.fieldType(c.ty)} = ")
          writeJavaConst(w, c.ty, c.value)
          w.wl
        }
      }
    }
  }

  private val moduleClass: String = spec.javaIdentStyle.ty(spec.moduleName) + "Module"

  override def generateModule(decls: Seq[InternTypeDecl]) {
    if (spec.jniUseOnLoad) {
      writeJavaFile(moduleClass, s"${spec.moduleName} Module", List.empty, w => {
        w.wl(s"public final class $moduleClass").braced {
          w.wl("static").braced {
            w.wl("if (System.getProperty(\"java.vm.vendor\").equals(\"The Android Project\"))").braced {
              w.wl("Guard.initialize();")
            }
          }
          w.wl("private static final class Guard").braced {
            w.wl("private static native void initialize();")
          }
        }
      })
    }
  }

  override def generateEnum(origin: String, ident: Ident, doc: Doc, e: Enum) {
    val refs = new JavaRefs()

    writeJavaFile(ident, origin, refs.java, w => {
      writeDoc(w, doc)
      javaAnnotationHeader.foreach(w.wl)
      w.w(s"${javaClassAccessModifierString}enum class ${marshal.typename(ident, e)}").braced {
        for (o <- normalEnumOptions(e)) {
          writeDoc(w, o.doc)
          w.wl(idJava.enum(o.ident) + ",")
        }
      }
    })
  }

  override def generateInterface(origin: String, ident: Ident, doc: Doc, typeParams: Seq[TypeParam], i: Interface) {
    val refs = new JavaRefs()

    i.methods.map(m => {
      m.params.map(p => refs.find(p.ty))
      m.ret.foreach(refs.find)
    })
    i.consts.map(c => {
      refs.find(c.ty)
    })
    if (i.ext.cpp) {
      refs.java.add("java.util.concurrent.atomic.AtomicBoolean")
      refs.java.add("com.snapchat.djinni.NativeObjectManager")
    }

    def writeModuleInitializer(w: IndentWriter) = {
      if (spec.jniUseOnLoad) {
        w.w("init").braced {
          w.w("try").braced {
            w.wl(s"Class.forName(${q(spec.javaPackage.getOrElse("") + "." + moduleClass)})")
          }
          w.w("catch (e: ClassNotFoundException)").braced {
            w.wl(s"throw IllegalStateException(${q("Failed to initialize djinni module")}, e)")
          }
        }
      }
    }

    writeJavaFile(ident, origin, refs.java, w => {
      val javaClass = marshal.typename(ident, i)
      val typeParamList = javaTypeParams(typeParams)
      writeDoc(w, doc)

      val statics = i.methods.filter(m => m.static && m.lang.java)

      javaAnnotationHeader.foreach(w.wl)

      // if no static and no cpp will use interface instead of abstract class
      val genJavaInterface = spec.javaGenInterface && !statics.nonEmpty && !i.ext.cpp
      val classOrInterfaceDesc = if (genJavaInterface) "interface" else "abstract class";
      val methodPrefixDesc = if (genJavaInterface) "" else "abstract ";

      w.w(s"${javaClassAccessModifierString}${classOrInterfaceDesc} $javaClass$typeParamList").braced {
        val skipFirst = SkipFirst()
        generateJavaConstants(w, i.consts)

        val throwException = spec.javaCppException.fold("")(" throws " + _)
        for (m <- i.methods if !m.static) {
          skipFirst { w.wl }
          writeMethodDoc(w, m, idJava.local)
          val ret = m.ret.fold("")(_ => ": " + marshal.returnType(m.ret))
          val params = m.params.map(p => idJava.local(p.ident) + ": " + marshal.paramType(p.ty)).mkString(", ")
          val meth = idJava.method(m.ident)
          w.wl(s"${methodPrefixDesc}fun $meth($params)$ret$throwException")
        }

        val statics = i.methods.filter(m => m.static && m.lang.java)
        if (statics.nonEmpty) {
          w.wl
          w.w("companion object").braced {
            writeModuleInitializer(w)
            for (m <- statics) {
              skipFirst {
                w.wl
              }
              writeMethodDoc(w, m, idJava.local)
              val ret = marshal.returnType(m.ret)
              val params = m.params.map(p => idJava.local(p.ident) + ": " + marshal.paramType(p.ty))
              w.wl("@kotlin.jvm.JvmStatic")
              w.wl("external fun " + idJava.method(m.ident) + params.mkString("(", ", ", ")") + ": " + ret)
            }
          } 
        }
        if (i.ext.cpp) {
          w.wl
          javaAnnotationHeader.foreach(w.wl)
          w.w(s"class CppProxy$typeParamList(private val nativeRef: Long) : $javaClass$typeParamList()").braced {
            writeModuleInitializer(w)
            w.wl("private val destroyed = AtomicBoolean(false)")
            w.wl
            w.w(s"init").braced {
              w.wl("if (nativeRef == 0L) throw RuntimeException(\"nativeRef is zero\")")
              w.wl("NativeObjectManager.register(this, nativeRef)")
            }
            w.wl
            w.w("companion object").braced {
              w.wl("@kotlin.jvm.JvmStatic")
              w.wl("external fun nativeDestroy(nativeRef: Long)")
            }
            for (m <- i.methods if !m.static) { // Static methods not in CppProxy
              val ret = m.ret.fold("")(_ => ": " + marshal.returnType(m.ret))
              val returnStmt = m.ret.fold("")(_ => "return ")
              val params = m.params.map(p => idJava.local(p.ident) + ": " + marshal.paramType(p.ty)).mkString(", ")
              val args = m.params.map(p => idJava.local(p.ident)).mkString(", ")
              val meth = idJava.method(m.ident)
              w.wl
              w.w(s"override fun $meth($params)$ret$throwException").braced {
                w.wl("assert(!destroyed.get()) { \"trying to use a destroyed object\" }")
                w.wl(s"${returnStmt}native_$meth(nativeRef${preComma(args)})")
              }
              w.wl(s"private external fun native_$meth(_nativeRef: Long${preComma(params)})$ret")
            }
          }
        }
      }
    })
  }

  override def generateRecord(origin: String, ident: Ident, doc: Doc, params: Seq[TypeParam], r: Record, idl: Seq[TypeDecl]) {
    val refs = new JavaRefs()
    r.fields.foreach(f => refs.find(f.ty))

    val javaName = if (r.ext.java) (ident.name + "_base") else ident.name
    val javaFinal = if (!r.ext.java && spec.javaUseFinalForRecord) "val " else "var "
    val superRecord = getSuperRecord(idl, r)
    val superFields: Seq[Field] = superRecord match {
      case None => Seq.empty
      case Some(value) => value.fields
    }

    writeJavaFile(javaName, origin, refs.java, w => {
      writeDoc(w, doc)
      javaAnnotationHeader.foreach(w.wl)
      val self = marshal.typename(javaName, r)

      val interfaces = scala.collection.mutable.ArrayBuffer[String]()
      if (r.derivingTypes.contains(DerivingType.Ord))
          interfaces += s"Comparable<$self>"
      if (r.derivingTypes.contains(DerivingType.AndroidParcelable)) {
          interfaces += "android.os.Parcelable"
          w.wl("@kotlinx.android.parcel.Parcelize")
      }
      val classOrDataClassDesc = if (r.fields.nonEmpty) "open class" else "class"
      val implementsSection = if (interfaces.isEmpty) "" else " : " + interfaces.mkString(", ")
      w.w(s"${javaClassAccessModifierString}$classOrDataClassDesc ${self + javaTypeParams(params)}")
      if (r.fields.nonEmpty) {
        w.wl("(")
        // Field definitions.
        var skipFirst = SkipFirst()
        for (f <- superFields) {
          skipFirst {
            w.wl(",")
          }
          w.w(s"    _${idJava.field(f.ident)}: ${marshal.fieldType(f.ty)}")
        }

        if (superFields.isEmpty) {
          skipFirst = SkipFirst()
        }
        
        for (f <- r.fields) {
          skipFirst {
            w.wl(",")
          }
          w.w(s"    val ${idJava.field(f.ident)}: ${marshal.fieldType(f.ty)}")
        }

        w.wl
        val extendsClass = marshal.extendsRecord(idl, r)
        w.w(s")$extendsClass$implementsSection")

        if (superFields.nonEmpty) {
          w.w("(")
          skipFirst = SkipFirst()
          for (f <- superFields) {
            skipFirst {
              w.w(",")
            }
            w.w(s"_${idJava.field(f.ident)}")
          }
          w.w(")")
        }
      }


      w.w(s"$implementsSection").braced {
        generateJavaConstants(w, r.consts)

        if (r.derivingTypes.contains(DerivingType.Eq)) {
          w.wl
          w.w(s"override fun equals(other: Any?): Boolean ").braced {
            w.wl("if (this === other) return true")
            w.wl("if (javaClass != other?.javaClass) return false")
            w.wl
            w.wl(s"other as $self")
            w.wl
            for (f <- superFields++r.fields) {
              f.ty.resolved.base match {
                case MBinary | MArray => w.w(s"if (!${idJava.field(f.ident)}.contentEquals(other.${idJava.field(f.ident)})) return false")
                case MList | MSet | MMap | MString | MDate | MOptional => w.wl(s"if (${idJava.field(f.ident)} != other.${idJava.field(f.ident)}) return false")
                case t: MPrimitive => w.wl(s"if (${idJava.field(f.ident)} != other.${idJava.field(f.ident)}) return false")
                case df: MDef => w.wl(s"if (${idJava.field(f.ident)} != other.${idJava.field(f.ident)}) return false")
                case e: MExtern => w.wl(s"if (${idJava.field(f.ident)} != other.${idJava.field(f.ident)}) return false")
                case _ => throw new AssertionError("Unreachable")
                }
            }
            w.wl
            w.wl("return true")
          }
          // Also generate a hashCode function, since you shouldn't override one without the other.
          // This hashcode implementation is based off of the apache commons-lang implementation of
          // HashCodeBuilder (excluding support for Java arrays) which is in turn based off of the
          // the recommendataions made in Effective Java.
          w.wl
          w.w("override fun hashCode(): Int ").braced {
            w.wl("// Pick an arbitrary non-zero starting value")
            w.wl("var hashCode = 17;")
            // Also pick an arbitrary prime to use as the multiplier.
            val multiplier = "31"
            for (f <- superFields++r.fields) {
              val fieldHashCode = f.ty.resolved.base match {
                case MBinary | MArray => s"${idJava.field(f.ident)}.contentHashCode()"
                case MList | MSet | MMap | MString | MDate => s"${idJava.field(f.ident)}.hashCode()"
                // Need to repeat this case for MDef
                case df: MDef => s"${idJava.field(f.ident)}.hashCode()"
                case MOptional => s"(${idJava.field(f.ident)}?.hashCode() ?: 0)"
                case t: MPrimitive => t.jName match {
                  case "byte" | "short" | "int" => idJava.field(f.ident)
                  case "long" | "float" | "double" | "boolean" => s"${idJava.field(f.ident)}.hashCode()"
                  case _ => throw new AssertionError("Unreachable")
                }
                case e: MExtern => e.defType match {
                  case DRecord => "(" + e.java.hash.format(idJava.field(f.ident)) + ")"
                  case DEnum => s"${idJava.field(f.ident)}.hashCode()"
                  case _ => throw new AssertionError("Unreachable")
                }
                case _ => throw new AssertionError("Unreachable")
              }
              w.wl(s"hashCode = hashCode * $multiplier + $fieldHashCode")
            }
            w.wl(s"return hashCode")
          }
        }

        w.wl
        w.w("override fun toString(): String ").braced {
          w.w(s"return ").nestedN(2) {
            w.wl(s""""${self} {" +""")
            var i: Int = 0
            for (f <- (superFields ++ r.fields)) {
              val name = idJava.field(f.ident)
              val comma = if (i > 0) """"," + """ else ""
              w.wl(s"""${comma}"${name}=" + ${name} +""")
              i+=1
            }
          }
          
          w.wl(s""""}"""")
        }
        w.wl

        if (r.derivingTypes.contains(DerivingType.Ord)) {
          def primitiveCompare(ident: Ident) {
            w.wl(s"if (this.${idJava.field(ident)} < other.${idJava.field(ident)}) {").nested {
              w.wl(s"tempResult = -1;")
            }
            w.wl(s"} else if (this.${idJava.field(ident)} > other.${idJava.field(ident)}) {").nested {
              w.wl(s"tempResult = 1;")
            }
            w.wl(s"} else {").nested {
              w.wl(s"tempResult = 0;")
            }
            w.wl("}")
          }
          w.wl
          w.w(s"override fun compareTo(other: $self): Int ").braced {
            w.wl("var tempResult: Int")
            for (f <- (superFields ++ r.fields)) {
              f.ty.resolved.base match {
                case MString | MDate => w.wl(s"tempResult = this.${idJava.field(f.ident)}.compareTo(other.${idJava.field(f.ident)});")
                case t: MPrimitive => primitiveCompare(f.ident)
                case df: MDef => df.defType match {
                  case DRecord => w.wl(s"tempResult = this.${idJava.field(f.ident)}.compareTo(other.${idJava.field(f.ident)});")
                  case DEnum => w.w(s"tempResult = this.${idJava.field(f.ident)}.compareTo(other.${idJava.field(f.ident)});")
                  case _ => throw new AssertionError("Unreachable")
                }
                case e: MExtern => e.defType match {
                  case DRecord => if(e.java.reference) w.wl(s"tempResult = this.${idJava.field(f.ident)}.compareTo(other.${idJava.field(f.ident)});") else primitiveCompare(f.ident)
                  case DEnum => w.w(s"tempResult = this.${idJava.field(f.ident)}.compareTo(other.${idJava.field(f.ident)});")
                  case _ => throw new AssertionError("Unreachable")
                }
                case _ => throw new AssertionError("Unreachable")
              }
              w.w("if (tempResult != 0)").braced {
                w.wl("return tempResult")
              }
            }
            w.wl("return 0")
          }
        }

      }
    })
  }

  def javaTypeParams(params: Seq[TypeParam]): String =
    if (params.isEmpty) "" else params.map(p => idJava.typeParam(p.ident)).mkString("<", ", ", ">")

}
