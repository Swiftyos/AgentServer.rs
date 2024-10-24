import math

def calculate_air_density(altitude):
    """
    Calculate the air density at a given altitude using the International Standard Atmosphere model.

    Parameters:
        altitude (float): Altitude in meters (must be less than 11,000 meters).

    Returns:
        tuple: Air density in kg/m^3, Air pressure in Pa, Air temperature in K.
    """
    # Constants
    P0 = 101325       # Sea-level standard atmospheric pressure, Pa
    T0 = 288.15       # Sea-level standard temperature, K
    L = 0.0065        # Temperature lapse rate, K/m
    R = 8.3144598     # Universal gas constant, J/(mol·K)
    g = 9.80665       # Gravitational acceleration, m/s^2
    M_air = 0.0289644 # Molar mass of dry air, kg/mol

    # Validate altitude
    if altitude < 0 or altitude >= 11000:
        raise ValueError("Altitude must be between 0 and 11,000 meters.")

    # Calculate temperature at altitude
    T = T0 - L * altitude

    # Calculate pressure at altitude
    exponent = (g * M_air) / (R * L)
    P = P0 * (1 - (L * altitude) / T0) ** exponent

    # Calculate air density at altitude
    rho = (P * M_air) / (R * T)
    return rho, P, T

def calculate_helium_density(P, T):
    """
    Calculate the density of helium at a given pressure and temperature.

    Parameters:
        P (float): Pressure in Pa.
        T (float): Temperature in K.

    Returns:
        float: Density of helium in kg/m^3.
    """
    R_specific = 2077  # Specific gas constant for helium, J/(kg·K)
    rho_helium = P / (R_specific * T)
    return rho_helium

def calculate_lifting_capacity(length, diameter, altitude, construction_option):
    """
    Calculate the maximum theoretical lifting capacity of an airship with different construction options.

    Parameters:
        length (float): Total length of the airship in meters.
        diameter (float): Diameter of the airship in meters.
        altitude (float): Altitude in meters (must be less than 11,000 meters).
        construction_option (str): Construction option selected.

    Returns:
        dict: A dictionary containing calculated values for each construction option.
    """
    # Validate construction option
    valid_options = ['Monocoque Shell', 'Geodesic Framework', 'Sandwich Panel', 'Tensegrity Structure', 'Helium-Filled Airship']
    if construction_option not in valid_options:
        raise ValueError(f"Invalid construction option. Choose from: {', '.join(valid_options)}")

    # Material properties
    materials = {
        'Monocoque Shell': {
            'E': 70e9,          # Young's modulus, Pa (e.g., carbon fiber composite)
            'rho': 1600,        # Material density, kg/m^3
            'nu': 0.3           # Poisson's ratio
        },
        'Geodesic Framework': {
            'E': 200e9,         # Young's modulus, Pa (e.g., high-strength steel)
            'rho': 7800,        # Material density, kg/m^3
            'nu': 0.3
        },
        'Sandwich Panel': {
            'E': 50e9,          # Effective Young's modulus, Pa
            'rho': 800,         # Effective density, kg/m^3 (lighter due to core)
            'nu': 0.3
        },
        'Tensegrity Structure': {
            'E': 70e9,          # Average modulus
            'rho': 1600,        # Average density
            'nu': 0.3
        },
        'Helium-Filled Airship': {
            'E': 5e9,           # Lower modulus due to reduced structural demands
            'rho': 1600,        # Material density, kg/m^3
            'nu': 0.3
        }
    }

    # Get material properties for selected construction option
    E = materials[construction_option]['E']
    rho_material = materials[construction_option]['rho']
    nu = materials[construction_option]['nu']
    safety_factor = 2  # Safety factor

    # Calculate radius
    radius = diameter / 2

    # Length of the cylindrical section
    L_cylinder = length - 2 * radius
    if L_cylinder < 0:
        raise ValueError("Length must be greater than twice the radius (diameter).")

    # Volume of the cylindrical section
    V_cylinder = math.pi * radius ** 2 * L_cylinder

    # Volume of the two hemispherical ends (equivalent to one full sphere)
    V_spheres = (4 / 3) * math.pi * radius ** 3

    # Total volume of the airship
    V_total = V_cylinder + V_spheres

    # Calculate air density, pressure, and temperature at the given altitude
    rho_air, P_air, T_air = calculate_air_density(altitude)

    # Initialize variables
    lifting_capacity = 0
    structural_mass = 0
    t_required = 0

    # Calculate structural mass and lifting capacity based on construction option
    if construction_option == 'Helium-Filled Airship':
        # Calculate helium density
        rho_helium = calculate_helium_density(P_air, T_air)

        # Calculate mass of air displaced and mass of helium
        m_air = rho_air * V_total
        m_helium = rho_helium * V_total

        # Maximum theoretical lifting capacity (buoyant force)
        lifting_capacity = m_air - m_helium

        # Structural mass estimation
        # Assume thinner shell due to reduced pressure difference
        t_required = 0.001  # Assume 1 mm thick shell

        # Surface area
        S_cylinder = 2 * math.pi * radius * L_cylinder
        S_spheres = 4 * math.pi * radius ** 2
        S_total = S_cylinder + S_spheres

        structural_mass = S_total * t_required * rho_material

    else:
        # For vacuum airships as before
        # Calculate the mass of air displaced (maximum theoretical lifting capacity)
        lifting_capacity = rho_air * V_total

        # Calculate surface area
        # Surface area of the cylindrical section
        S_cylinder = 2 * math.pi * radius * L_cylinder

        # Surface area of the two hemispherical ends (equivalent to one full sphere)
        S_spheres = 4 * math.pi * radius ** 2

        # Total surface area
        S_total = S_cylinder + S_spheres

        if construction_option == 'Monocoque Shell':
            # Monocoque Shell calculations
            # Calculate required wall thickness to prevent buckling under external pressure
            # Using the buckling formula for spherical shells
            t_sphere = radius * math.sqrt((P_air * math.sqrt(3 * (1 - nu ** 2))) / (2 * E))

            # Using the buckling formula for cylindrical shells
            t_cylinder = radius * math.sqrt((P_air) / (E * (1 - nu ** 2)))

            # Take the maximum required thickness
            t_required = max(t_sphere, t_cylinder) * safety_factor

            # Calculate structural mass
            structural_mass = S_total * t_required * rho_material

        elif construction_option == 'Geodesic Framework':
            # Geodesic Framework calculations
            # Assume struts form a geodesic dome structure over the surface
            # Estimate mass based on a percentage of the monocoque shell mass
            # For example, 50% of monocoque shell mass
            monocoque_mass = S_total * (radius * math.sqrt((P_air) / (E * (1 - nu ** 2)))) * rho_material
            structural_mass = 0.5 * monocoque_mass

            # Required thickness is not directly applicable
            t_required = None

        elif construction_option == 'Sandwich Panel':
            # Sandwich Panel calculations
            # Assume a thinner skin with a lightweight core
            skin_thickness = 0.001  # 1 mm skin thickness
            core_thickness = 0.01   # 10 mm core thickness
            t_required = skin_thickness * 2 + core_thickness

            # Effective density
            effective_density = rho_material  # Assume overall lower density due to core

            # Structural mass
            structural_mass = S_total * t_required * effective_density

        elif construction_option == 'Tensegrity Structure':
            # Tensegrity Structure calculations
            # Estimate mass based on a percentage of the monocoque shell mass
            # For example, 40% of monocoque shell mass
            monocoque_mass = S_total * (radius * math.sqrt((P_air) / (E * (1 - nu ** 2)))) * rho_material
            structural_mass = 0.4 * monocoque_mass

            # Required thickness is not directly applicable
            t_required = None

    # Calculate net payload capacity
    net_payload = lifting_capacity - structural_mass

    return {
        'volume': V_total,
        'surface_area': S_total,
        'air_density': rho_air,
        'lifting_capacity': lifting_capacity,
        'structural_mass': structural_mass,
        'net_payload': net_payload,
        'required_thickness': t_required,
        'construction_option': construction_option
    }

def main():
    print("Airship Lifting Capacity Calculator with Construction Options")
    print("--------------------------------------------------------------")
    try:
        # User inputs
        length = float(input("Enter the total length of the airship in meters: "))
        diameter = float(input("Enter the diameter of the airship in meters: "))
        altitude = float(input("Enter the altitude in meters (0 - 11,000 m): "))

        # Construction options
        print("\nConstruction Options:")
        print("1. Monocoque Shell")
        print("2. Geodesic Framework")
        print("3. Sandwich Panel")
        print("4. Tensegrity Structure")
        print("5. Helium-Filled Airship")
        option_input = input("Select a construction option (1-5) or 'all' to compare all options: ")

        if option_input.lower() == 'all':
            options = ['Monocoque Shell', 'Geodesic Framework', 'Sandwich Panel', 'Tensegrity Structure', 'Helium-Filled Airship']
        else:
            option_map = {
                '1': 'Monocoque Shell',
                '2': 'Geodesic Framework',
                '3': 'Sandwich Panel',
                '4': 'Tensegrity Structure',
                '5': 'Helium-Filled Airship'
            }
            if option_input not in option_map:
                raise ValueError("Invalid option selected.")
            options = [option_map[option_input]]

        # Calculate and display results for selected options
        for construction_option in options:
            results = calculate_lifting_capacity(length, diameter, altitude, construction_option)

            # Display the results
            print(f"\nResults for {construction_option}:")
            print(f"At an altitude of {altitude} meters:")
            print(f"- Airship Volume: {results['volume']:.2f} cubic meters")
            print(f"- Airship Surface Area: {results['surface_area']:.2f} square meters")
            print(f"- Air Density: {results['air_density']:.4f} kg/m³")
            print(f"- Maximum Theoretical Lifting Capacity (Buoyant Force): {results['lifting_capacity']:.2f} kg")

            if results['required_thickness'] is not None:
                print(f"- Required Wall Thickness: {results['required_thickness'] * 1000:.2f} mm")
            else:
                print("- Required Wall Thickness: Not applicable")

            print(f"- Estimated Structural Mass: {results['structural_mass']:.2f} kg")
            print(f"- Net Payload Capacity: {results['net_payload']:.2f} kg")

        print("\nNote: For the Helium-Filled Airship, the lifting capacity accounts for the mass of helium inside.")

    except ValueError as e:
        print(f"\nError: {e}")

if __name__ == "__main__":
    main()