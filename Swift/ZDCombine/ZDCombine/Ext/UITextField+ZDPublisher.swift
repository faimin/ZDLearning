//
//  UITextField+ZDPublisher.swift
//  ZDCombine
//
//  Created by Zero.D.Saber on 2024/9/21.
//

import UIKit
import Combine

extension UITextField {
    var textPublisher: AnyPublisher<String, Never> {
        NotificationCenter.default
            .publisher(for: UITextField.textDidChangeNotification, object: self)
            .compactMap({ $0.object as? Self })
            .compactMap(\.text)
            .eraseToAnyPublisher()
    }
}
