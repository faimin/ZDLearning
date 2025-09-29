//
//  InitialPublished.swift
//  ZDCombine
//
//  Created by Zero.D.Saber on 2024/9/21.
//

import Combine

@propertyWrapper
struct InitialPublished<Value>: Publisher {
    typealias Output = Value
    typealias Failure = Never
    
    private let subject: CurrentValueSubject<Output, Failure>
    
    var wrappedValue: Value {
        get { subject.value }
        set { subject.send(newValue) }
    }
    
    init(wrappedValue initialValue: Value) {
        subject = CurrentValueSubject(initialValue)
    }
    
    func receive<S>(subscriber: S) where S : Subscriber, Never == S.Failure, Value == S.Input {
        subject.receive(subscriber: subscriber)
    }
}
