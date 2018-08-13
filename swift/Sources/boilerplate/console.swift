//
//  console.swift
//  Culprit
//
//  Created by Wess Cope on 1/22/18.
//  Copyright © 2018 WessCope. All rights reserved.
//

import Foundation

struct console {
  private static let infoCharacter    = "ℹ️"
  private static let debugCharacter   = "🐞"
  private static let successCharacter = "✅"
  private static let warningCharacter = "⚠️"
  private static let errorCharacter   = "❗️"
  private static let fatalCharacter   = "‼️"

  static let log = console.info
  
  static func info(_ args:String...) {
    console.safePrint(
      (infoCharacter + " : " + buildMessage(args))
    )
  }
  
  static func debug(_ args:String...) {
    console.safePrint(
      (debugCharacter + " : " + buildMessage(args))
    )
  }
  
  static func success(_ args:String...) {
    console.safePrint(
      (successCharacter + " : " + buildMessage(args))
    )
  }
  
  static func warning(_ args:String...) {
    console.safePrint(
      (warningCharacter + " : " + buildMessage(args))
    )
  }
  
  static func error(_ args:String...) {
    console.safePrint(
      (errorCharacter + " : " + buildMessage(args))
    )
  }
  
  private static func buildMessage(_ args:[Any]) -> String {
    let parts = args.map { part in
      return "\(part)"
    }
    
    return parts.joined(separator: " ")
  }
  
  private static func safePrint(_ message:String) {
    #if DEBUG
      
      print(message)
      
    #endif
  }
}

