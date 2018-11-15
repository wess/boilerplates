//
//  console.swift
//
//  Created by Wess Cope on 1/22/18.
//  Copyright © 2018 WessCope. All rights reserved.
//

public struct console {
  private static let infoCharacter    = "ℹ️ "
  private static let debugCharacter   = "🐞 "
  private static let successCharacter = "✅ "
  private static let warningCharacter = "⚠️ "
  private static let errorCharacter   = "❗️ "
  private static let fatalCharacter   = "‼️ "


  public static func info(_ args:Any..., line:Int = #line, file:String = #file, function:String = #function) {
    #if DEBUG
    console.safePrint((infoCharacter + " \(line) : \((file.split(separator: "/").last ?? "")) : \(function) :: " + buildMessage(args)))
    #else
    safePrint(buildMessage(args))
    #endif
  }

  public static func log(_ args:Any..., line:Int = #line, file:String = #file, function:String = #function) {
    #if DEBUG
    console.safePrint((infoCharacter + " \(line) : \((file.split(separator: "/").last ?? "")) : \(function) :: " + buildMessage(args)))
    #else
    safePrint(buildMessage(args))
    #endif
  }

  public static func debug(_ args:Any..., line:Int = #line, file:String = #file, function:String = #function) {
    #if DEBUG
    console.safePrint((debugCharacter + " \(line) : \((file.split(separator: "/").last ?? "")) : \(function) :: " + buildMessage(args)))
    #else
    safePrint(buildMessage(args))
    #endif
  }
  
  public static func success(_ args:Any..., line:Int = #line, file:String = #file, function:String = #function) {
    #if DEBUG
    console.safePrint((successCharacter + " \(line) : \((file.split(separator: "/").last ?? "")) : \(function) :: " + buildMessage(args)))
    #else
    safePrint(buildMessage(args))
    #endif
  }
  
  public static func warning(_ args:Any..., line:Int = #line, file:String = #file, function:String = #function) {
    #if DEBUG
    console.safePrint((warningCharacter + " \(line) : \((file.split(separator: "/").last ?? "")) : \(function) :: " + buildMessage(args)))
    #else
    safePrint(buildMessage(args))
    #endif
  }
  
  public static func error(_ args:Any..., line:Int = #line, file:String = #file, function:String = #function) {
    #if DEBUG
    console.safePrint((errorCharacter + " \(line) : \((file.split(separator: "/").last ?? "")) : \(function) :: " + buildMessage(args)))
    #else
    safePrint(buildMessage(args))
    #endif
  }
  
  private static func buildMessage(_ args:[Any]) -> String {
    let parts = args.map { part in
      return "\(part)"
    }
    
    return parts.joined(separator: " ")
  }
  
  private static func safePrint(_ message:String) {
    print(message)
  }
}
