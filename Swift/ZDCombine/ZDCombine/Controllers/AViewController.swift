//
//  AViewController.swift
//  ZDCombine
//
//  Created by Zero.D.Saber on 2024/9/21.
//

import UIKit
import Combine
import CombineCocoa
import CombineExt


class AViewController: UIViewController {
    
    @IBOutlet weak var textFiled: UITextField!
    private var binds = Set<AnyCancellable>()
    
    deinit {
        print(#file, #function, #line)
    }
    
    required init?(coder: NSCoder) {
        //fatalError("init(coder:) has not been implemented")
        super.init(coder: coder)
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        
        textFiled.textPublisher
            .throttle(for: 1, scheduler: RunLoop.main, latest: true)
            .removeDuplicates()
            .compactMap({ $0 })
            .filter({ $0.isEmpty == false })
            .sink { value in
                print("text = \(value)")
            }
            .store(in: &binds)
    }

    
}
