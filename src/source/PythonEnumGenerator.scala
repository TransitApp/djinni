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

class PythonEnumGenerator(spec: Spec) extends Generator(spec) {

  var tripleq = "\"\"\""

  override def generateEnum(origin: String, ident: Ident, doc: Doc, e: Enum) {
    if (e.flags) {
      return // Not Implemented
    }

    createFile(spec.pythonEnumOutFolder.get, idPython.file(ident) + ".py", (w: IndentWriter) => {
      writeDoc(w, doc)

      w.wl(s"from enum import Enum")
      w.wl

      w.wl(s"class ${idPython.ty(ident.name)}(Enum):").nested {
        writeEnumOptions(w, e, idPython.enum, "=")
        w.wl
        w.wl("def __int__(self) -> int:").nested {
          w.wl(s"return self.value[0]")
        }
        w.wl
        w.wl("def __str__(self) -> str:").nested {
          w.wl(s"return self.value[1]")
        }
      }

      w.wl
    })
  }

  /** Write the enum options for a Python Enum with support for C++ names.
   * Instead of generating a normal enum with ordinal values, it handles both ordinal values and C++ names. This allows
   * us to store the _name_ in resource files (tiles) and use the from_string to convert it back. This protects us from
   * invalidation if the enum's ordering changes.
   */
  override def writeEnumOptions(w: IndentWriter, e: Enum, ident: IdentConverter, delim: String = "=") {
    
    var shift = 0
    for (o <- normalEnumOptions(e)) {
      w.wl(ident(o.ident.name) + s" $delim $shift, " + '"' + idCpp.enum(o.ident.name) + '"')
      writeDoc(w, o.doc)
      shift += 1
    }
  }

  override def writeDoc(w: IndentWriter, doc: Doc) {
    doc.lines.length match {
      case 0 =>
      case 1 =>
        w.wl(tripleq + doc.lines.head+ tripleq)
      case _ =>
        w.wl(tripleq)
        doc.lines.foreach (l => w.wl(s" *$l"))
        w.wl(tripleq)
    }
  }

  override def generateRecord(origin: String, ident: Ident, doc: Doc, params: Seq[TypeParam], r: Record, idl: Seq[TypeDecl]) {
    return // Not Implemented
  }

  override def generateInterface(origin: String, ident: Ident, doc: Doc, typeParams: Seq[TypeParam], i: Interface) {
    return // Not Implemented
  }
}
